#[path = "modules/cpu.rs"]
mod cpu;
#[path = "modules/de.rs"]
mod de;
#[path = "modules/host.rs"]
mod host;
#[path = "modules/kernel.rs"]
mod kernel;
#[path = "modules/mem.rs"]
mod mem;
#[path = "modules/os.rs"]
mod os;
#[path = "modules/pkg.rs"]
mod pkg;
#[path = "modules/res.rs"]
mod res;
#[path = "modules/shell.rs"]
mod sh;
#[path = "modules/uptime.rs"]
mod up;
#[path = "modules/user.rs"]
mod user;

pub fn info() {
    match (user::get_user(), host::get_host()) {
        (Ok(user), Ok(hostname)) => {
            println!("            {}@{}", user, hostname);
        }
        (Err(err), _) => eprintln!("Error getting OS: {}", err),
        (_, Err(err)) => eprintln!("Error getting hostname: {}", err),
    }

    if let Ok(os) = os::get_os() {
        println!("OS        : {}", os);
    }

    if let Ok(kernel) = kernel::get_kernel() {
        println!("Kernel    : {}", kernel);
    }

    if let Ok(uptime) = up::get_uptime() {
        println!("Uptime    : {}", uptime);
    }

    println!();

    if let Ok(de_session) = de::get_de() {
        println!("DE        : {}", de_session);
    }

    if let Ok(shell) = sh::get_shell() {
        println!("Shell     : {}", shell);
    }

    if let Ok(resolution) = res::get_res() {
        println!("Resolution: {}", resolution);
    }

    println!();

    if let Ok(packages) = pkg::get_pkg() {
        println!("Packages  : {}", packages);
    }

    if let Ok(cpu_model) = cpu::get_cpu() {
        println!("CPU       : {}", cpu_model);
    }

    if let Ok(memory) = mem::get_mem() {
        println!("Memory    : {}", memory);
    }
}