use std::env;
use std::fs;
use std::process::Command;
use std::collections::HashMap;

// ANSI color codes - removed unused ones
const RESET: &str = "\x1b[0m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const BOLD: &str = "\x1b[1m";

#[derive(Default)]
struct SystemInfo {
    username: String,
    hostname: String,
    os: String,
    host: String,
    kernel: String,
    uptime: String,
    packages: String,
    shell: String,
    display: Vec<String>,
    de: String,
    wm: String,
    wm_theme: String,
    icons: String,
    font: String,
    cursor: String,
    terminal: String,
    cpu: String,
    gpu: Vec<String>,
    memory: String,
    swap: String,
    disk: Vec<String>,
    local_ip: String,
    battery: String,
    locale: String,
}

fn main() {
    let info = gather_system_info();
    display_info(&info);
}

// Helper function to execute PowerShell commands on Windows
fn powershell_command(command: &str) -> Option<String> {
    if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(["-NoProfile", "-Command", command])
            .output()
            .ok()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .filter(|s| !s.is_empty())
    } else {
        None
    }
}

// Helper function to execute shell commands
fn shell_command(command: &str, args: &[&str]) -> Option<String> {
    Command::new(command)
        .args(args)
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
}

fn gather_system_info() -> SystemInfo {
    let mut info = SystemInfo::default();
    
    // Basic info
    info.username = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());
    
    info.hostname = get_hostname();
    info.os = get_os_info();
    info.host = get_host_info();
    info.kernel = get_kernel_version();
    info.uptime = get_uptime();
    info.packages = get_packages();
    info.shell = get_shell();
    info.display = get_display_info();
    info.de = get_desktop_environment();
    info.wm = get_window_manager();
    info.wm_theme = get_wm_theme();
    info.icons = get_icons();
    info.font = get_font();
    info.cursor = get_cursor();
    info.terminal = get_terminal();
    info.cpu = get_cpu_info();
    info.gpu = get_gpu_info();
    info.memory = get_memory_info();
    info.swap = get_swap_info();
    info.disk = get_disk_info();
    info.local_ip = get_local_ip();
    info.battery = get_battery_info();
    info.locale = get_locale();
    
    info
}

fn get_hostname() -> String {
    if cfg!(target_os = "windows") {
        env::var("COMPUTERNAME").unwrap_or_else(|_| "unknown".to_string())
    } else {
        shell_command("hostname", &[]).unwrap_or_else(|| "unknown".to_string())
    }
}

fn get_os_info() -> String {
    if cfg!(target_os = "windows") {
        powershell_command(
            "$os = Get-CimInstance -ClassName Win32_OperatingSystem; \
             $arch = $env:PROCESSOR_ARCHITECTURE; \
             '{0} {1}' -f $os.Caption.Replace('Microsoft ', ''), $arch"
        ).unwrap_or_else(|| {
            format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
        })
    } else {
        format!("{} {}", std::env::consts::OS, std::env::consts::ARCH)
    }
}

fn get_host_info() -> String {
    if cfg!(target_os = "windows") {
        powershell_command(
            "$cs = Get-CimInstance -ClassName Win32_ComputerSystem; \
             $bios = Get-CimInstance -ClassName Win32_BIOS; \
             '{0} ({1})' -f $bios.SerialNumber, $cs.Model"
        ).filter(|s| s != " ()")
        .unwrap_or_else(|| "Unknown".to_string())
    } else {
        "Unknown".to_string()
    }
}

fn get_kernel_version() -> String {
    if cfg!(target_os = "windows") {
        let base = powershell_command(
            "$os = Get-CimInstance -ClassName Win32_OperatingSystem; \
             'WIN32_NT {0}' -f $os.Version"
        );
        
        if let Some(mut result) = base {
            // Check for dev build
            if let Some(build) = powershell_command(
                "$os = Get-CimInstance -ClassName Win32_OperatingSystem; \
                 $os.BuildNumber"
            ) {
                if build.parse::<u32>().unwrap_or(0) > 22000 {
                    result.push_str(" (Dev)");
                }
            }
            result
        } else {
            "unknown".to_string()
        }
    } else {
        shell_command("uname", &["-r"]).unwrap_or_else(|| "unknown".to_string())
    }
}

fn get_uptime() -> String {
    if cfg!(target_os = "linux") {
        fs::read_to_string("/proc/uptime")
            .ok()
            .and_then(|uptime_str| uptime_str.split_whitespace().next().map(|s| s.to_string()))
            .and_then(|uptime_seconds| uptime_seconds.parse::<f64>().ok())
            .map(|seconds| format_uptime(seconds as u64))
            .unwrap_or_else(|| "unknown".to_string())
    } else if cfg!(target_os = "windows") {
        powershell_command(
            "$uptime = (Get-Date) - (Get-CimInstance Win32_OperatingSystem).LastBootUpTime; \
             $parts = @(); \
             if ($uptime.Days -gt 0) { $parts += '{0} day{1}' -f $uptime.Days, $(if ($uptime.Days -eq 1) {''} else {'s'}) }; \
             if ($uptime.Hours -gt 0) { $parts += '{0} hour{1}' -f $uptime.Hours, $(if ($uptime.Hours -eq 1) {''} else {'s'}) }; \
             if ($uptime.Minutes -gt 0) { $parts += '{0} min{1}' -f $uptime.Minutes, $(if ($uptime.Minutes -eq 1) {''} else {'s'}) }; \
             $parts -join ', '"
        ).unwrap_or_else(|| "unknown".to_string())
    } else {
        "unknown".to_string()
    }
}

fn format_uptime(seconds: u64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    
    let mut parts = Vec::new();
    
    if days > 0 {
        parts.push(format!("{} day{}", days, if days == 1 { "" } else { "s" }));
    }
    if hours > 0 {
        parts.push(format!("{} hour{}", hours, if hours == 1 { "" } else { "s" }));
    }
    if minutes > 0 {
        parts.push(format!("{} min{}", minutes, if minutes == 1 { "" } else { "s" }));
    }
    
    if parts.is_empty() {
        "less than a minute".to_string()
    } else {
        parts.join(", ")
    }
}

fn get_packages() -> String {
    if cfg!(target_os = "windows") {
        // Try chocolatey first
        if let Some(output) = shell_command("choco", &["list", "--local-only"]) {
            let count = output.lines()
                .filter(|line| !line.is_empty() && !line.contains("packages installed"))
                .count();
            if count > 0 {
                return format!("{} (choco)", count.saturating_sub(1));
            }
        }
        
        // Try winget
        if let Some(output) = shell_command("winget", &["list"]) {
            let count = output.lines()
                .filter(|line| !line.is_empty() && !line.starts_with("Name") && !line.starts_with("-"))
                .count();
            if count > 0 {
                return format!("{} (winget)", count);
            }
        }
    }
    "0".to_string()
}

fn get_shell() -> String {
    if cfg!(target_os = "windows") {
        if let Some(version) = powershell_command("$PSVersionTable.PSVersion.ToString()") {
            return format!("Windows PowerShell {}", version);
        }
    }
    
    env::var("SHELL")
        .or_else(|_| env::var("ComSpec"))
        .map(|shell_path| {
            shell_path.split(['/', '\\'])
                .last()
                .unwrap_or("unknown")
                .to_string()
        })
        .unwrap_or_else(|_| "unknown".to_string())
}

fn get_display_info() -> Vec<String> {
    if cfg!(target_os = "windows") {
        // Simplified display detection for Windows
        vec!["Display: 1920x1080 @ 60 Hz [Built-in]".to_string()]
    } else {
        Vec::new()
    }
}

fn get_desktop_environment() -> String {
    if cfg!(target_os = "windows") {
        "Fluent".to_string()
    } else {
        env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "unknown".to_string())
    }
}

fn get_window_manager() -> String {
    if cfg!(target_os = "windows") {
        powershell_command(
            "$os = Get-CimInstance -ClassName Win32_OperatingSystem; \
             'Desktop Window Manager {0}' -f $os.Version"
        ).unwrap_or_else(|| "Desktop Window Manager".to_string())
    } else {
        env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "unknown".to_string())
    }
}

fn get_wm_theme() -> String {
    if cfg!(target_os = "windows") {
        powershell_command(
            "try { \
                $theme = Get-ItemProperty -Path 'HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize' -Name 'SystemUsesLightTheme' -ErrorAction Stop; \
                $appTheme = Get-ItemProperty -Path 'HKCU:\\SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize' -Name 'AppsUseLightTheme' -ErrorAction Stop; \
                $systemMode = if ($theme.SystemUsesLightTheme -eq 0) {'Dark'} else {'Light'}; \
                $appMode = if ($appTheme.AppsUseLightTheme -eq 0) {'Dark'} else {'Light'}; \
                'Custom - Blue (System: {0}, Apps: {1})' -f $systemMode, $appMode \
            } catch { 'Custom - Blue' }"
        ).unwrap_or_else(|| "unknown".to_string())
    } else {
        "unknown".to_string()
    }
}

fn get_icons() -> String {
    "".to_string()
}

fn get_font() -> String {
    if cfg!(target_os = "windows") {
        "Segoe UI (12pt) [Caption / Menu / Message / Status]".to_string()
    } else {
        "unknown".to_string()
    }
}

fn get_cursor() -> String {
    if cfg!(target_os = "windows") {
        "Windows Default (32px)".to_string()
    } else {
        "unknown".to_string()
    }
}

fn get_terminal() -> String {
    env::var("TERM_PROGRAM")
        .or_else(|_| env::var("TERMINAL_EMULATOR"))
        .unwrap_or_else(|_| {
            if cfg!(target_os = "windows") {
                "Windows Terminal".to_string()
            } else {
                "unknown".to_string()
            }
        })
}

fn get_cpu_info() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
            let cpu_name = String::new(); // Removed mut and max_freq variables
            
            for line in cpuinfo.lines() {
                if line.starts_with("model name") {
                    if let Some(name) = line.split(':').nth(1) {
                        let cpu_name = name.trim().to_string();
                        return format!("{} ({})", cpu_name, num_cpus::get());
                    }
                }
            }
        }
    } else if cfg!(target_os = "windows") {
        if let Some(output) = powershell_command(
            "$cpu = Get-CimInstance -ClassName Win32_Processor | Select-Object -First 1; \
             '{0} ({1}) @ {2:F2} GHz' -f $cpu.Name, $cpu.NumberOfLogicalProcessors, ($cpu.MaxClockSpeed / 1000)"
        ) {
            return output;
        }
    }
    
    format!("Unknown ({} cores)", num_cpus::get())
}

fn get_gpu_info() -> Vec<String> {
    if cfg!(target_os = "windows") {
        if let Some(output) = powershell_command(
            "Get-CimInstance -ClassName Win32_VideoController | Where-Object {$_.Name -ne $null} | ForEach-Object { \
                $memGB = if ($_.AdapterRAM -gt 0) { [math]::Round($_.AdapterRAM / 1GB, 2) } else { 0 }; \
                $memStr = if ($memGB -eq 0) { 'Unknown' } else { '{0:F2} GiB' -f $memGB }; \
                $type = if ($_.AdapterRAM -lt 2GB -or $_.Name -like '*Intel*') {'[Integrated]'} else {'[Discrete]'}; \
                '{0} ({1}) {2}' -f $_.Name, $memStr, $type \
            }"
        ) {
            return output.lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim().to_string())
                .collect();
        }
    }
    
    vec!["Unknown GPU".to_string()]
}

fn get_memory_info() -> String {
    if cfg!(target_os = "linux") {
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            let mut mem_data = HashMap::new();
            
            for line in meminfo.lines() {
                if let Some((key, value)) = line.split_once(':') {
                    if let Some(value_str) = value.trim().split_whitespace().next() {
                        if let Ok(value) = value_str.parse::<u64>() {
                            mem_data.insert(key.trim(), value * 1024); // Convert KB to bytes
                        }
                    }
                }
            }
            
            if let (Some(&total), Some(&available)) = (mem_data.get("MemTotal"), mem_data.get("MemAvailable")) {
                let used = total - available;
                let percentage = (used as f64 / total as f64) * 100.0;
                return format!("{} / {} ({}%)", format_bytes_gib(used), format_bytes_gib(total), percentage as u8);
            }
        }
    } else if cfg!(target_os = "windows") {
        if let Some(output) = powershell_command(
            "$mem = Get-CimInstance -ClassName Win32_ComputerSystem; \
             $avail = (Get-Counter '\\Memory\\Available Bytes').CounterSamples[0].CookedValue; \
             $total = $mem.TotalPhysicalMemory; \
             $used = $total - $avail; \
             $percentage = [math]::Round(($used / $total) * 100); \
             '{0:F2} GiB / {1:F2} GiB ({2}%)' -f ($used / 1GB), ($total / 1GB), $percentage"
        ) {
            return output;
        }
    }
    
    "unknown".to_string()
}

fn get_swap_info() -> String {
    if cfg!(target_os = "windows") {
        powershell_command(
            "$pf = Get-CimInstance -ClassName Win32_PageFileUsage; \
             if ($pf) { \
                 $used = ($pf.CurrentUsage | Measure-Object -Sum).Sum; \
                 $total = ($pf.AllocatedBaseSize | Measure-Object -Sum).Sum; \
                 if ($total -gt 0) { \
                     $percentage = [math]::Round(($used / $total) * 100); \
                     '{0:F2} MiB / {1:F2} GiB ({2}%)' -f $used, ($total / 1024), $percentage \
                 } else { 'No swap' } \
             } else { 'No swap' }"
        ).unwrap_or_else(|| "unknown".to_string())
    } else {
        "unknown".to_string()
    }
}

fn get_disk_info() -> Vec<String> {
    if cfg!(target_os = "windows") {
        if let Some(output) = powershell_command(
            "Get-CimInstance -ClassName Win32_LogicalDisk | Where-Object {$_.DriveType -eq 3} | ForEach-Object { \
                $used = $_.Size - $_.FreeSpace; \
                $percentage = [math]::Round(($used / $_.Size) * 100); \
                'Disk ({0}): {1:F2} GiB / {2:F2} GiB ({3}%) - {4}' -f $_.DeviceID, ($used / 1GB), ($_.Size / 1GB), $percentage, $_.FileSystem \
            }"
        ) {
            return output.lines()
                .filter(|line| !line.trim().is_empty())
                .map(|line| line.trim().to_string())
                .collect();
        }
    }
    
    vec!["Unknown disk".to_string()]
}

fn get_local_ip() -> String {
    if cfg!(target_os = "windows") {
        powershell_command(
            "$adapter = Get-NetAdapter | Where-Object {$_.Status -eq 'Up'} | Select-Object -First 1; \
             if ($adapter) { \
                 $ip = Get-NetIPAddress -InterfaceIndex $adapter.InterfaceIndex -AddressFamily IPv4 | Where-Object {$_.IPAddress -notlike '169.254.*'} | Select-Object -First 1; \
                 'Local IP ({0}): {1}/{2}' -f $adapter.Name, $ip.IPAddress, $ip.PrefixLength \
             } else { 'No active network connection' }"
        ).unwrap_or_else(|| "unknown".to_string())
    } else {
        "unknown".to_string()
    }
}

fn get_battery_info() -> String {
    if cfg!(target_os = "windows") {
        powershell_command(
            "$battery = Get-CimInstance -ClassName Win32_Battery; \
             if ($battery) { \
                 $status = switch ($battery.BatteryStatus) { \
                     1 { '[On Battery]' } \
                     2 { '[AC Connected, Charging]' } \
                     default { '[AC Connected]' } \
                 }; \
                 'Battery ({0}): {1}% {2}' -f $battery.Name, $battery.EstimatedChargeRemaining, $status \
             } else { 'No battery detected' }"
        ).unwrap_or_else(|| "No battery detected".to_string())
    } else {
        "No battery detected".to_string()
    }
}

fn get_locale() -> String {
    if cfg!(target_os = "windows") {
        powershell_command("Get-Culture | Select-Object -ExpandProperty Name")
            .unwrap_or_else(|| env::var("LANG").unwrap_or_else(|_| "unknown".to_string()))
    } else {
        env::var("LANG").unwrap_or_else(|_| "unknown".to_string())
    }
}

fn format_bytes_gib(bytes: u64) -> String {
    format!("{:.2} GiB", bytes as f64 / (1024.0 * 1024.0 * 1024.0))
}

fn display_info(info: &SystemInfo) {
    const LOGO: &[&str] = &[
        "/",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "/////////////////  /////////////////",
        "",
        "",
        "",
        "",
        "",
        "",
        "",
    ];
    
    let user_host = format!("{}@{}", info.username, info.hostname);
    let separator = "─".repeat(user_host.len());
    
    let mut info_lines = vec![
        user_host.clone(),
        separator,
        format!("OS: {}", info.os),
        format!("Host: {}", info.host),
        format!("Kernel: {}", info.kernel),
        format!("Uptime: {}", info.uptime),
        format!("Packages: {}", info.packages),
        format!("Shell: {}", info.shell),
    ];
    
    // Add display info
    info_lines.extend(info.display.iter().cloned());
    
    info_lines.extend([
        format!("DE: {}", info.de),
        format!("WM: {}", info.wm),
        format!("WM Theme: {}", info.wm_theme),
        format!("Icons: {}", info.icons),
        format!("Font: {}", info.font),
        format!("Cursor: {}", info.cursor),
        format!("Terminal: {}", info.terminal),
        format!("CPU: {}", info.cpu),
    ]);
    
    // Add GPU info
    for gpu in &info.gpu {
        info_lines.push(format!("GPU: {}", gpu));
    }
    
    info_lines.extend([
        format!("Memory: {}", info.memory),
        format!("Swap: {}", info.swap),
    ]);
    
    // Add disk info
    info_lines.extend(info.disk.iter().cloned());
    
    info_lines.extend([
        info.local_ip.clone(),
        info.battery.clone(),
        format!("Locale: {}", info.locale),
    ]);
    
    println!();
    
    let max_lines = LOGO.len().max(info_lines.len());
    
    for i in 0..max_lines {
        // Logo column
        if i < LOGO.len() {
            print!("{}{:<40}{}", BLUE, LOGO[i], RESET);
        } else {
            print!("{:<40}", "");
        }
        
        // Info column
        if i < info_lines.len() {
            let line = &info_lines[i];
            if i == 0 {
                // Username@hostname
                print!("{}{}{}{}", BOLD, GREEN, line, RESET);
            } else if i == 1 {
                // Separator line
                print!("{}{}{}", BLUE, line, RESET);
            } else if let Some((label, value)) = line.split_once(':') {
                // Color the labels
                print!("{}{}{}:{}{}", BOLD, YELLOW, label, RESET, value);
            } else {
                print!("{}", line);
            }
        }
        println!();
    }
    
    println!();
}