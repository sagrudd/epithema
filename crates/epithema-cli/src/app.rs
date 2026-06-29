//! Top-level CLI application orchestration for `epithema`.

use std::collections::{HashMap, HashSet};
use std::ffi::OsString;
use std::io::{self, IsTerminal, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use clap::{CommandFactory, Parser, Subcommand};
use epithema_service::{
    EpithemaService, HttpDownloadProgress, HttpDownloadProgressState, InvocationRequest,
    NgsDownloadProgress, NgsDownloadProgressCallback, NgsDownloadProgressContext, ServiceRegistry,
    ToolName,
};
use epithema_tools::governed_tool_descriptors;

use crate::commands;
use crate::error::CliError;
use crate::output;

/// CLI application wrapper for the governed `epithema` binary.
pub struct CliApp;

impl CliApp {
    /// Parses arguments, executes the requested command, and returns the final
    /// process exit code.
    #[must_use]
    pub fn run() -> ExitCode {
        match Cli::try_parse() {
            Ok(cli) => match Self::run_parsed(cli) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("epithema: {error}");
                    error.exit_code()
                }
            },
            Err(error) => error.exit(),
        }
    }

    fn run_parsed(cli: Cli) -> Result<(), CliError> {
        let service = build_service();

        match cli.command {
            None => {
                print!("{}", Cli::command().render_long_help());
                Ok(())
            }
            Some(Command::List) => {
                commands::run_list(&service);
                Ok(())
            }
            Some(Command::Autodoc {
                input,
                emit_docs,
                docs_output_dir,
                emit_validation_stub,
                validation_output_path,
            }) => commands::run_autodoc(
                &service,
                &input,
                emit_docs,
                docs_output_dir.as_deref(),
                emit_validation_stub,
                validation_output_path.as_deref(),
            )
            .map(|_| ())
            .map_err(CliError::from),
            Some(Command::Tool(arguments)) => Self::run_tool(&service, arguments),
        }
    }

    fn run_tool(service: &EpithemaService, arguments: Vec<OsString>) -> Result<(), CliError> {
        let mut arguments = arguments.into_iter();
        let tool = arguments
            .next()
            .and_then(|value| value.into_string().ok())
            .ok_or_else(CliError::missing_tool_name)?;
        let tool_arguments = arguments
            .map(|value| {
                value.into_string().map_err(|_| {
                    CliError::from(
                        epithema_diagnostics::PlatformError::new(
                            epithema_diagnostics::ErrorCategory::Validation,
                            "tool arguments must be valid UTF-8",
                        )
                        .with_code("cli.tool.arguments.non_utf8"),
                    )
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        let tool = ToolName::new(tool).map_err(CliError::from)?;
        let request =
            InvocationRequest::new(service.default_context(), tool).with_arguments(tool_arguments);
        let response = service.invoke(request).map_err(CliError::from)?;

        output::print_tool_response(&response, service);
        Ok(())
    }
}

fn build_service() -> EpithemaService {
    let mut registry = ServiceRegistry::new();
    for descriptor in governed_tool_descriptors() {
        registry
            .register(*descriptor)
            .expect("built-in tool registration should succeed");
    }
    EpithemaService::new(registry).with_ngs_download_progress(ngs_download_progress_callback())
}

fn ngs_download_progress_callback() -> Arc<NgsDownloadProgressCallback> {
    let state = Arc::new(Mutex::new(ProgressDashboardState::new(
        io::stderr().is_terminal(),
    )));
    Arc::new(move |progress: NgsDownloadProgress| {
        let output = match state.lock() {
            Ok(mut state) => state.record(progress, Instant::now()),
            Err(_) => return,
        };
        if let Some(output) = output {
            let mut stderr = io::stderr().lock();
            let _ = stderr.write_all(output.as_bytes());
            let _ = stderr.flush();
        }
    })
}

#[derive(Debug)]
struct ProgressDashboardState {
    entries: HashMap<String, ProgressRenderState>,
    order: Vec<String>,
    rendered_lines: usize,
    dashboard_enabled: bool,
    activity_frame: usize,
    summary: Option<ProgressSummaryState>,
}

impl ProgressDashboardState {
    fn new(dashboard_enabled: bool) -> Self {
        Self {
            entries: HashMap::new(),
            order: Vec::new(),
            rendered_lines: 0,
            dashboard_enabled,
            activity_frame: 0,
            summary: None,
        }
    }

    fn record(&mut self, progress: NgsDownloadProgress, now: Instant) -> Option<String> {
        let key = progress.transfer.url.clone();
        if let Some(context) = &progress.context {
            self.record_summary(context, &key, &progress.transfer);
        }
        if progress.summary_only {
            if self.dashboard_enabled {
                let lines = self.rendered_lines();
                let output = render_progress_dashboard(&lines, self.rendered_lines);
                self.rendered_lines = lines.len();
                return Some(output);
            }
            return None;
        }
        let previous = self.entries.get(&key);
        let speed_bytes_per_second = previous.and_then(|previous| {
            let elapsed = now.duration_since(previous.last_rendered);
            if elapsed.is_zero() {
                None
            } else {
                Some(
                    progress
                        .transfer
                        .bytes_downloaded
                        .saturating_sub(previous.bytes_downloaded) as f64
                        / elapsed.as_secs_f64(),
                )
            }
        });
        let should_render = match (progress.transfer.state, previous) {
            (HttpDownloadProgressState::Started | HttpDownloadProgressState::Finished, _) => true,
            (
                HttpDownloadProgressState::Finalizing | HttpDownloadProgressState::Verifying,
                None,
            ) => true,
            (
                HttpDownloadProgressState::Finalizing | HttpDownloadProgressState::Verifying,
                Some(previous),
            ) => now.duration_since(previous.last_rendered) >= Duration::from_secs(2),
            (HttpDownloadProgressState::Advanced, None) => true,
            (HttpDownloadProgressState::Advanced, Some(previous)) => {
                now.duration_since(previous.last_rendered) >= Duration::from_secs(2)
            }
        };
        if !should_render {
            return None;
        }

        if !self.entries.contains_key(&key) {
            self.order.push(key.clone());
        }
        let label = progress
            .transfer
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("download")
            .to_owned();
        let entry_key = key.clone();
        let finished_key = (progress.transfer.state == HttpDownloadProgressState::Finished)
            .then(|| entry_key.clone());
        let activity_frame = self.activity_frame;
        if progress.transfer.state != HttpDownloadProgressState::Finished {
            self.activity_frame = self.activity_frame.wrapping_add(1);
        }
        let phase_started_at = previous
            .filter(|previous| previous.latest_progress.state == progress.transfer.state)
            .map(|previous| previous.phase_started_at)
            .unwrap_or(now);
        let transfer = progress.transfer;
        self.entries.insert(
            key,
            ProgressRenderState {
                last_rendered: now,
                phase_started_at,
                bytes_downloaded: transfer.bytes_downloaded,
                latest_progress: transfer,
                speed_bytes_per_second,
                label,
                activity_frame,
            },
        );

        if self.dashboard_enabled {
            let lines = self.rendered_lines();
            let output = render_progress_dashboard(&lines, self.rendered_lines);
            self.rendered_lines = lines.len();
            self.retire_finished(finished_key);
            Some(output)
        } else {
            let output = self.entries.get(&entry_key).map(|entry| {
                format!(
                    "{}\n",
                    render_ngs_download_progress(
                        &entry.label,
                        &entry.latest_progress,
                        entry.speed_bytes_per_second,
                        entry.activity_frame,
                        entry.phase_elapsed(now),
                    )
                )
            });
            self.retire_finished(finished_key);
            output
        }
    }

    fn rendered_lines(&self) -> Vec<String> {
        let mut lines = self
            .summary
            .as_ref()
            .map(render_ngs_download_summary)
            .unwrap_or_default();
        lines.extend(
            self.order
                .iter()
                .filter_map(|key| self.entries.get(key))
                .map(|entry| {
                    render_ngs_download_progress(
                        &entry.label,
                        &entry.latest_progress,
                        entry.speed_bytes_per_second,
                        entry.activity_frame,
                        entry.phase_elapsed(Instant::now()),
                    )
                })
                .collect::<Vec<_>>(),
        );
        lines
    }

    fn retire_finished(&mut self, finished_key: Option<String>) {
        if let Some(finished_key) = finished_key {
            self.entries.remove(&finished_key);
            self.order.retain(|key| key != &finished_key);
        }
    }

    fn record_summary(
        &mut self,
        context: &NgsDownloadProgressContext,
        asset_key: &str,
        transfer: &HttpDownloadProgress,
    ) {
        let summary = self
            .summary
            .get_or_insert_with(|| ProgressSummaryState::from_context(context));
        summary.merge_context(context);
        summary.record_asset(asset_key, context, transfer);
    }
}

#[derive(Clone, Debug)]
struct ProgressRenderState {
    last_rendered: Instant,
    phase_started_at: Instant,
    bytes_downloaded: u64,
    latest_progress: HttpDownloadProgress,
    speed_bytes_per_second: Option<f64>,
    label: String,
    activity_frame: usize,
}

impl ProgressRenderState {
    fn phase_elapsed(&self, now: Instant) -> Duration {
        now.duration_since(self.phase_started_at)
    }
}

#[derive(Clone, Debug)]
struct ProgressSummaryState {
    accession: String,
    provider: String,
    title: Option<String>,
    run_count: usize,
    sample_count: usize,
    selected_asset_count: usize,
    selected_total_bytes: Option<u64>,
    completed_assets: HashSet<String>,
    asset_bytes: HashMap<String, u64>,
    runs: HashMap<String, ProgressGroupState>,
    samples: HashMap<String, ProgressGroupState>,
}

impl ProgressSummaryState {
    fn from_context(context: &NgsDownloadProgressContext) -> Self {
        Self {
            accession: context.accession.clone(),
            provider: context.provider.clone(),
            title: context.title.clone(),
            run_count: context.run_count,
            sample_count: context.sample_count,
            selected_asset_count: context.selected_asset_count,
            selected_total_bytes: context.selected_total_bytes,
            completed_assets: HashSet::new(),
            asset_bytes: HashMap::new(),
            runs: HashMap::new(),
            samples: HashMap::new(),
        }
    }

    fn merge_context(&mut self, context: &NgsDownloadProgressContext) {
        self.accession = context.accession.clone();
        self.provider = context.provider.clone();
        if self.title.is_none() {
            self.title = context.title.clone();
        }
        self.run_count = context.run_count;
        self.sample_count = context.sample_count;
        self.selected_asset_count = context.selected_asset_count;
        self.selected_total_bytes = context.selected_total_bytes;
    }

    fn record_asset(
        &mut self,
        asset_key: &str,
        context: &NgsDownloadProgressContext,
        transfer: &HttpDownloadProgress,
    ) {
        let observed_bytes = self
            .asset_bytes
            .get(asset_key)
            .copied()
            .unwrap_or_default()
            .max(transfer.bytes_downloaded);
        self.asset_bytes
            .insert(asset_key.to_owned(), observed_bytes);
        let run_group = self
            .runs
            .entry(context.run_accession.clone())
            .or_insert_with(|| ProgressGroupState::new(context.run_selected_asset_count));
        run_group.total_assets = context.run_selected_asset_count;

        let sample_key = context
            .sample_accession
            .clone()
            .unwrap_or_else(|| context.run_accession.clone());
        let sample_group = self
            .samples
            .entry(sample_key)
            .or_insert_with(|| ProgressGroupState::new(context.sample_selected_asset_count));
        sample_group.total_assets = context.sample_selected_asset_count;

        if transfer.state == HttpDownloadProgressState::Finished {
            let asset_key = asset_key.to_owned();
            self.completed_assets.insert(asset_key.clone());
            run_group.completed_assets.insert(asset_key.clone());
            sample_group.completed_assets.insert(asset_key);
        }
    }

    fn completed_runs(&self) -> usize {
        self.runs
            .values()
            .filter(|group| group.is_complete())
            .count()
    }

    fn completed_samples(&self) -> usize {
        self.samples
            .values()
            .filter(|group| group.is_complete())
            .count()
    }

    fn observed_bytes(&self) -> u64 {
        self.asset_bytes.values().copied().sum()
    }
}

#[derive(Clone, Debug)]
struct ProgressGroupState {
    total_assets: usize,
    completed_assets: HashSet<String>,
}

impl ProgressGroupState {
    fn new(total_assets: usize) -> Self {
        Self {
            total_assets,
            completed_assets: HashSet::new(),
        }
    }

    fn is_complete(&self) -> bool {
        self.total_assets > 0 && self.completed_assets.len() >= self.total_assets
    }
}

fn render_progress_dashboard(lines: &[String], previous_line_count: usize) -> String {
    if lines.is_empty() {
        return String::new();
    }

    let mut output = String::new();
    if previous_line_count > 0 {
        output.push_str(&format!("\x1b[{previous_line_count}F"));
    }
    for index in 0..previous_line_count.max(lines.len()) {
        output.push_str("\x1b[2K");
        if let Some(line) = lines.get(index) {
            output.push_str(line);
        }
        output.push('\n');
    }
    output
}

fn render_ngs_download_summary(summary: &ProgressSummaryState) -> Vec<String> {
    const BAR_WIDTH: usize = 16;
    let title = summary
        .title
        .as_deref()
        .map(|title| format!("  title {}", shorten_text(title, 84)))
        .unwrap_or_default();
    let completed_assets = summary
        .completed_assets
        .len()
        .min(summary.selected_asset_count);
    let observed_bytes = summary
        .selected_total_bytes
        .map(|total| summary.observed_bytes().min(total));
    let progress_ratio = if let (Some(observed), Some(total)) = (
        observed_bytes,
        summary.selected_total_bytes.filter(|total| *total > 0),
    ) {
        observed as f64 / total as f64
    } else if summary.selected_asset_count > 0 {
        completed_assets as f64 / summary.selected_asset_count as f64
    } else {
        0.0
    };
    let filled = (progress_ratio * BAR_WIDTH as f64).round() as usize;
    let empty = BAR_WIDTH.saturating_sub(filled);
    let percent = progress_ratio * 100.0;
    let size = summary
        .selected_total_bytes
        .map(|bytes| {
            format!(
                "  bytes {} / {}",
                format_bytes(observed_bytes.unwrap_or(0)),
                format_bytes(bytes)
            )
        })
        .unwrap_or_default();

    vec![
        format!(
            "ngsget study {}  provider {}{}",
            summary.accession, summary.provider, title
        ),
        format!(
            "ngsget overall assets {}/{} [{}{}] {:>6.2}%  runs {}/{}  samples {}/{}{}",
            completed_assets,
            summary.selected_asset_count,
            "=".repeat(filled),
            " ".repeat(empty),
            percent,
            summary.completed_runs(),
            summary.run_count,
            summary.completed_samples(),
            summary.sample_count,
            size,
        ),
    ]
}

fn render_ngs_download_progress(
    label: &str,
    progress: &HttpDownloadProgress,
    speed_bytes_per_second: Option<f64>,
    activity_frame: usize,
    phase_elapsed: Duration,
) -> String {
    const BAR_WIDTH: usize = 16;
    const SPINNER_FRAMES: &[&str] = &["|", "/", "-", "\\"];
    let short_label = shorten_text(label, 36);
    let activity = match progress.state {
        HttpDownloadProgressState::Finished => "done".to_owned(),
        HttpDownloadProgressState::Finalizing => "wait".to_owned(),
        HttpDownloadProgressState::Verifying => "md5".to_owned(),
        HttpDownloadProgressState::Started | HttpDownloadProgressState::Advanced => {
            SPINNER_FRAMES[activity_frame % SPINNER_FRAMES.len()].to_owned()
        }
    };
    let phase = match progress.state {
        HttpDownloadProgressState::Finalizing | HttpDownloadProgressState::Verifying => {
            format!("  elapsed {}", format_duration(phase_elapsed))
        }
        _ => String::new(),
    };

    if let Some(total) = progress.total_bytes.filter(|total| *total > 0) {
        let ratio = (progress.bytes_downloaded as f64 / total as f64).clamp(0.0, 1.0);
        let filled = (ratio * BAR_WIDTH as f64).round() as usize;
        let empty = BAR_WIDTH.saturating_sub(filled);
        let percent = ratio * 100.0;
        format!(
            "ngsget {activity:<4} {short_label:<36} [{}{}] {:>6.2}% {} / {}  speed {}{}",
            "=".repeat(filled),
            " ".repeat(empty),
            percent,
            format_bytes(progress.bytes_downloaded),
            format_bytes(total),
            format_speed(speed_bytes_per_second),
            phase,
        )
    } else {
        format!(
            "ngsget {activity:<4} {short_label:<36} {} downloaded  speed {}{}",
            format_bytes(progress.bytes_downloaded),
            format_speed(speed_bytes_per_second),
            phase,
        )
    }
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let seconds = seconds % 60;
    if hours > 0 {
        format!("{hours:02}:{minutes:02}:{seconds:02}")
    } else {
        format!("{minutes:02}:{seconds:02}")
    }
}

fn shorten_text(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_owned();
    }
    let suffix_len = max_chars.saturating_sub(3);
    let suffix: String = value
        .chars()
        .rev()
        .take(suffix_len)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect();
    format!("...{suffix}")
}

fn format_speed(bytes_per_second: Option<f64>) -> String {
    bytes_per_second
        .map(|speed| format!("{}/s", format_bytes(speed.max(0.0) as u64)))
        .unwrap_or_else(|| "calculating".to_owned())
}

fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit_index = 0usize;
    while value >= 1024.0 && unit_index + 1 < UNITS.len() {
        value /= 1024.0;
        unit_index += 1;
    }
    if unit_index == 0 {
        format!("{bytes} {}", UNITS[unit_index])
    } else {
        format!("{value:.2} {}", UNITS[unit_index])
    }
}

#[derive(Debug, Parser)]
#[command(
    name = "epithema",
    version,
    about = "Governed EMBOSS reboot command surface in Rust.",
    long_about = "Epithema is a governed reboot of EMBOSS with a single binary surface. Use `epithema <tool>` for tool execution, `epithema list` for governed tool discovery, and `epithema autodoc` to validate autodoc contracts, emit generated documentation pages, and derive validation evidence stubs.",
    arg_required_else_help = false,
    disable_help_subcommand = true
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// List currently governed tools known to the shared service layer.
    List,
    /// Load and validate an autodoc JSON contract, optionally emitting generated docs pages.
    Autodoc {
        /// Path to the autodoc JSON document to load and validate.
        input: PathBuf,
        /// Emit generated Markdown pages under the docs tree.
        #[arg(long)]
        emit_docs: bool,
        /// Override the output directory used for generated Markdown pages.
        #[arg(long)]
        docs_output_dir: Option<PathBuf>,
        /// Emit a structured validation evidence stub as JSON.
        #[arg(long)]
        emit_validation_stub: bool,
        /// Override the output path used for the validation evidence JSON.
        #[arg(long)]
        validation_output_path: Option<PathBuf>,
    },
    #[command(external_subcommand)]
    /// Invoke a governed Epithema tool through the shared service layer.
    Tool(Vec<OsString>),
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::time::{Duration, Instant};

    use clap::Parser;

    use super::{
        Cli, HttpDownloadProgress, HttpDownloadProgressState, NgsDownloadProgress,
        NgsDownloadProgressContext, ProgressDashboardState, render_ngs_download_progress,
        render_progress_dashboard,
    };

    fn ngs_progress(
        transfer: HttpDownloadProgress,
        context: Option<NgsDownloadProgressContext>,
    ) -> NgsDownloadProgress {
        NgsDownloadProgress {
            transfer,
            context,
            summary_only: false,
        }
    }

    fn ngs_summary_progress(
        transfer: HttpDownloadProgress,
        context: Option<NgsDownloadProgressContext>,
    ) -> NgsDownloadProgress {
        NgsDownloadProgress {
            transfer,
            context,
            summary_only: true,
        }
    }

    fn summary_context(
        run_accession: &str,
        sample_accession: &str,
        run_selected_asset_count: usize,
        sample_selected_asset_count: usize,
    ) -> NgsDownloadProgressContext {
        NgsDownloadProgressContext {
            accession: "PRJEB50706".to_owned(),
            provider: "ena".to_owned(),
            title: Some("Porkchop study".to_owned()),
            run_count: 2,
            sample_count: 2,
            selected_asset_count: 3,
            selected_total_bytes: Some(300),
            run_accession: run_accession.to_owned(),
            sample_accession: Some(sample_accession.to_owned()),
            run_selected_asset_count,
            sample_selected_asset_count,
        }
    }

    #[test]
    fn renders_ngs_download_progress_with_clear_size_and_speed_columns() {
        let progress = HttpDownloadProgress {
            state: HttpDownloadProgressState::Advanced,
            url: "https://example.invalid/BMDM_mrna1273_0h_3.tar.gz".to_owned(),
            path: PathBuf::from("BMDM_mrna1273_0h_3.tar.gz.partial"),
            bytes_downloaded: 4_370_129_224,
            total_bytes: Some(71_811_853_189),
        };

        let rendered = render_ngs_download_progress(
            "BMDM_mrna1273_0h_3.tar.gz.partial",
            &progress,
            Some(13_107_200.0),
            0,
            Duration::ZERO,
        );

        assert!(rendered.contains("4.07 GiB / 66.88 GiB  speed 12.50 MiB/s"));
        assert!(rendered.starts_with("ngsget |"));
        assert!(!rendered.contains("GiBGiB"));
    }

    #[test]
    fn renders_ngs_post_transfer_phase_elapsed_time() {
        let progress = HttpDownloadProgress {
            state: HttpDownloadProgressState::Verifying,
            url: "https://example.invalid/ERR8562402.fastq.gz".to_owned(),
            path: PathBuf::from("ERR8562402.fastq.gz.partial"),
            bytes_downloaded: 4_370_129_224,
            total_bytes: Some(8_685_137_920),
        };

        let rendered = render_ngs_download_progress(
            "ERR8562402.fastq.gz.partial",
            &progress,
            Some(40_000_000.0),
            0,
            Duration::from_secs(125),
        );

        assert!(rendered.starts_with("ngsget md5"));
        assert!(rendered.contains("elapsed 02:05"));
        assert!(rendered.contains("speed 38.15 MiB/s"));
    }

    #[test]
    fn repaints_active_download_rows_on_heartbeat_without_byte_delta() {
        let mut dashboard = ProgressDashboardState::new(true);
        let started_at = Instant::now();
        let progress = HttpDownloadProgress {
            state: HttpDownloadProgressState::Started,
            url: "https://example.invalid/a.fastq.gz".to_owned(),
            path: PathBuf::from("a.fastq.gz.partial"),
            bytes_downloaded: 0,
            total_bytes: Some(100),
        };

        let first_render = dashboard
            .record(ngs_progress(progress.clone(), None), started_at)
            .expect("started download should render");
        let mut heartbeat = progress;
        heartbeat.state = HttpDownloadProgressState::Advanced;
        let heartbeat_render = dashboard
            .record(
                ngs_progress(heartbeat, None),
                started_at + Duration::from_secs(2),
            )
            .expect("heartbeat should repaint even without byte progress");

        assert!(first_render.contains("ngsget |"));
        assert!(heartbeat_render.contains("ngsget /"));
        assert!(heartbeat_render.contains("speed 0 B/s"));
    }

    #[test]
    fn renders_ngs_download_dashboard_by_repainting_existing_rows() {
        let lines = vec![
            "ngsget file_a.partial [==                      ]".to_owned(),
            "ngsget file_b.partial [=                       ]".to_owned(),
        ];

        let rendered = render_progress_dashboard(&lines, 2);

        assert!(rendered.starts_with("\x1b[2F"));
        assert_eq!(rendered.matches("\x1b[2K").count(), 2);
        assert_eq!(rendered.matches("ngsget file_a.partial").count(), 1);
        assert_eq!(rendered.matches("ngsget file_b.partial").count(), 1);
    }

    #[test]
    fn retires_finished_download_rows_after_rendering_completion_once() {
        let mut dashboard = ProgressDashboardState::new(true);
        let started_at = Instant::now();
        let file_a = HttpDownloadProgress {
            state: HttpDownloadProgressState::Started,
            url: "https://example.invalid/a.fastq.gz".to_owned(),
            path: PathBuf::from("a.fastq.gz.partial"),
            bytes_downloaded: 45,
            total_bytes: Some(100),
        };
        let file_b = HttpDownloadProgress {
            state: HttpDownloadProgressState::Started,
            url: "https://example.invalid/b.fastq.gz".to_owned(),
            path: PathBuf::from("b.fastq.gz.partial"),
            bytes_downloaded: 0,
            total_bytes: Some(100),
        };

        dashboard.record(ngs_progress(file_a.clone(), None), started_at);
        dashboard.record(
            ngs_progress(file_b.clone(), None),
            started_at + Duration::from_millis(1),
        );
        let mut finished_a = file_a;
        finished_a.state = HttpDownloadProgressState::Finished;
        finished_a.bytes_downloaded = 100;
        let finished_render = dashboard
            .record(
                ngs_progress(finished_a, None),
                started_at + Duration::from_secs(1),
            )
            .expect("finished download should render once");
        assert!(finished_render.contains("a.fastq.gz.partial"));

        let mut advanced_b = file_b;
        advanced_b.state = HttpDownloadProgressState::Advanced;
        advanced_b.bytes_downloaded = 50;
        let next_render = dashboard
            .record(
                ngs_progress(advanced_b, None),
                started_at + Duration::from_secs(3),
            )
            .expect("remaining active download should render");
        assert!(!next_render.contains("a.fastq.gz.partial"));
        assert!(next_render.contains("b.fastq.gz.partial"));
    }

    #[test]
    fn renders_ngs_download_dashboard_with_study_summary() {
        let mut dashboard = ProgressDashboardState::new(true);
        let started_at = Instant::now();
        let file_a = HttpDownloadProgress {
            state: HttpDownloadProgressState::Started,
            url: "https://example.invalid/a.fastq.gz".to_owned(),
            path: PathBuf::from("a.fastq.gz.partial"),
            bytes_downloaded: 45,
            total_bytes: Some(100),
        };
        let mut file_b = HttpDownloadProgress {
            state: HttpDownloadProgressState::Finished,
            url: "https://example.invalid/b.fastq.gz".to_owned(),
            path: PathBuf::from("b.fastq.gz.partial"),
            bytes_downloaded: 100,
            total_bytes: Some(100),
        };

        let first_render = dashboard
            .record(
                ngs_progress(file_a, Some(summary_context("ERR1", "ERS1", 2, 2))),
                started_at,
            )
            .expect("started download should render");
        let finished_render = dashboard
            .record(
                ngs_progress(file_b.clone(), Some(summary_context("ERR2", "ERS2", 1, 1))),
                started_at + Duration::from_secs(1),
            )
            .expect("finished download should render");
        file_b.url = "https://example.invalid/a.fastq.gz".to_owned();
        let completed_render = dashboard
            .record(
                ngs_progress(file_b, Some(summary_context("ERR1", "ERS1", 2, 2))),
                started_at + Duration::from_secs(2),
            )
            .expect("second finished download should render");

        assert!(
            first_render.contains("ngsget study PRJEB50706  provider ena  title Porkchop study")
        );
        assert!(first_render.contains("assets 0/3"));
        assert!(first_render.contains("15.00%"), "{first_render:?}");
        assert!(first_render.contains("bytes 45 B / 300 B"));
        assert!(finished_render.contains("assets 1/3"));
        assert!(finished_render.contains("runs 1/2"));
        assert!(finished_render.contains("samples 1/2"));
        assert!(completed_render.contains("assets 2/3"));
        assert!(completed_render.contains("runs 1/2"));
        assert!(completed_render.contains("samples 1/2"));
    }

    #[test]
    fn renders_summary_only_baseline_without_transfer_rows() {
        let mut dashboard = ProgressDashboardState::new(true);
        let started_at = Instant::now();
        let baseline = HttpDownloadProgress {
            state: HttpDownloadProgressState::Finished,
            url: "https://example.invalid/a.fastq.gz".to_owned(),
            path: PathBuf::from("a.fastq.gz"),
            bytes_downloaded: 100,
            total_bytes: Some(100),
        };

        let rendered = dashboard
            .record(
                ngs_summary_progress(baseline, Some(summary_context("ERR1", "ERS1", 1, 2))),
                started_at,
            )
            .expect("summary-only baseline should render dashboard summary");

        assert!(rendered.contains("assets 1/3"));
        assert!(rendered.contains("33.33%"));
        assert!(rendered.contains("bytes 100 B / 300 B"));
        assert!(!rendered.contains("a.fastq.gz"));
    }

    #[test]
    fn parses_list_command() {
        let cli = Cli::try_parse_from(["epithema", "list"]).expect("list should parse");
        assert!(format!("{cli:?}").contains("List"));
    }

    #[test]
    fn parses_autodoc_command() {
        let cli = Cli::try_parse_from(["epithema", "autodoc", "example.json"])
            .expect("autodoc should parse");
        assert!(format!("{cli:?}").contains("Autodoc"));
        assert!(format!("{cli:?}").contains("example.json"));
    }

    #[test]
    fn parses_autodoc_generation_flags() {
        let cli = Cli::try_parse_from([
            "epithema",
            "autodoc",
            "example.json",
            "--emit-docs",
            "--docs-output-dir",
            "docs/generated",
            "--emit-validation-stub",
            "--validation-output-path",
            "docs/generated/validation/example.validation.json",
        ])
        .expect("autodoc generation flags should parse");
        assert!(format!("{cli:?}").contains("emit_docs: true"));
        assert!(format!("{cli:?}").contains("emit_validation_stub: true"));
        assert!(format!("{cli:?}").contains("docs/generated"));
    }

    #[test]
    fn routes_unknown_subcommand_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "needle"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("needle"));
    }

    #[test]
    fn routes_seqret_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "seqret", "ena:AB000263"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("seqret"));
    }

    #[test]
    fn routes_seqretsetall_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "seqretsetall", "a.fasta", "b.fasta"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("seqretsetall"));
    }

    #[test]
    fn routes_seqretsplit_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "seqretsplit", "a.fasta"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("seqretsplit"));
    }

    #[test]
    fn routes_refseqget_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "refseqget", "ncbi:protein:NP_000537.3"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("refseqget"));
    }

    #[test]
    fn routes_assemblyget_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "assemblyget", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("assemblyget"));
    }

    #[test]
    fn routes_whichdb_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "whichdb", "ena:AB000263"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("whichdb"));
    }

    #[test]
    fn routes_runinfo_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "runinfo", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("runinfo"));
    }

    #[test]
    fn routes_infoassembly_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "infoassembly", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("infoassembly"));
    }

    #[test]
    fn routes_runget_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "runget", "ena:ERR123456"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("runget"));
    }

    #[test]
    fn routes_ngslist_to_tool_path() {
        let cli = Cli::try_parse_from([
            "epithema",
            "ngslist",
            "PRJNA1011899",
            "--provider",
            "ena",
            "--format",
            "json",
        ])
        .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("ngslist"));
    }

    #[test]
    fn routes_ngsget_to_tool_path() {
        let cli = Cli::try_parse_from([
            "epithema",
            "ngsget",
            "PRJNA1011899",
            "--provider",
            "ena",
            "--out",
            "ngs-output",
            "--raw",
            "--threads",
            "3",
            "--transport",
            "aspera",
            "--ascp",
            "/opt/aspera/bin/ascp",
            "--aspera-key",
            "/opt/aspera/etc/asperaweb_id_dsa.openssh",
            "--aspera-rate",
            "1g",
            "--check-downloads",
            "existing-downloads",
        ])
        .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("ngsget"));
    }

    #[test]
    fn routes_charge_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "charge", "example.faa"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("charge"));
    }

    #[test]
    fn routes_density_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "density", "example.fna"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("density"));
    }

    #[test]
    fn routes_banana_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "banana", "example.fna"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("banana"));
    }

    #[test]
    fn routes_wobble_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "wobble", "example.fna"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("wobble"));
    }

    #[test]
    fn routes_isochore_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "isochore", "example.fna"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("isochore"));
    }

    #[test]
    fn routes_syco_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "syco", "example.fna"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("syco"));
    }

    #[test]
    fn routes_hmoment_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "hmoment", "example.faa"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("hmoment"));
    }

    #[test]
    fn routes_octanol_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "octanol", "example.faa"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("octanol"));
    }

    #[test]
    fn routes_pepinfo_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "pepinfo", "example.faa"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("pepinfo"));
    }

    #[test]
    fn routes_psiphi_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "psiphi", "example.pdb"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("psiphi"));
    }

    #[test]
    fn routes_primersearch_to_tool_path() {
        let cli = Cli::try_parse_from([
            "epithema",
            "primersearch",
            "targets.fasta",
            "primer_pairs.tsv",
        ])
        .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("primersearch"));
    }

    #[test]
    fn routes_eprimer3_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "eprimer3", "targets.fasta"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("eprimer3"));
    }

    #[test]
    fn routes_sirna_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "sirna", "targets.fasta"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("sirna"));
    }

    #[test]
    fn routes_wossname_to_tool_path() {
        let cli = Cli::try_parse_from(["epithema", "wossname", "pairwise align"])
            .expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("wossname"));
    }

    #[test]
    fn routes_seealso_to_tool_path() {
        let cli =
            Cli::try_parse_from(["epithema", "seealso", "needle"]).expect("tool should parse");
        assert!(format!("{cli:?}").contains("Tool"));
        assert!(format!("{cli:?}").contains("seealso"));
    }
}
