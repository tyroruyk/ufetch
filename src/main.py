import platform
import os
import time
import psutil

os_name = platform.system()
os_arch = platform.machine()
os_ver = platform.version()
os_hostname = platform.node()
os_release = platform.release()

yellow='\033[93m'
green='\033[92m'
cyan='\033[96m'
pink='\033[95m'
red='\033[91m'
b='\033[1m'

def uptime_in_linux():
    uptime = time.time() - psutil.boot_time()
    uptime_hour = int(uptime / 3600)
    uptime_min = int((uptime % 3600) / 60)
    uptime_sec = int(uptime % 60)

    return f"{uptime_hour}:{uptime_min}:{uptime_sec}"

def ram_usage_in_linux():
    if os_name == 'Linux':
        ram_usage = os.popen('free -m | grep Mem | awk \'{print $3/$2 * 100.0}\'').read()
        ram_usage = ram_usage.strip()
        return ram_usage
    else:
        return 'Unknown'

def get_local_time_zone():
    if os_name == 'Linux':
        tz = os.popen('cat /etc/timezone').read()
        tz = tz.strip()
        return tz
    elif os_name == 'Darwin':
        tz = os.popen('systemsetup -gettimezone').read()
        tz = tz.strip()
        return tz
    else:
        return 'Unknown'

def default_shell_in_linux():
    if os_name == 'Linux':
        shell = os.popen('echo $SHELL').read()
        shell = shell.strip()
        return shell
    else:
        return 'Unknown'

def processor_usage_linux():
    if os_name == 'Linux':
        cpu_usage = os.popen('top -bn1 | grep "Cpu(s)" | awk -F" " \'{print $2 + $4}\'').read()
        cpu_usage = cpu_usage.strip()
        return cpu_usage
    else:
        return 'Unknown'

de = str(os.environ.get('DESKTOP_SESSION'))
de = de.upper()

if __name__ == "__main__":

    f = open("./output.txt", "w")

    f.write(b+cyan+f"   OS: {os_ver}\n\n")
    f.write(f"   Architecture: {os_arch}\n\n")
    f.write(f"   Kernal: {os_name} {os_release}\n\n")
    f.write(f"   Hostname: {os_hostname}\n\n")
    f.write(f"   RAM Usage: {ram_usage_in_linux()}%\n\n")
    f.write(f"   Desktop Environment: {de}\n\n")
    f.write(f"   CPU Usage: {processor_usage_linux()}%\n\n")
    f.write(f"   Timezone: {get_local_time_zone()}\n\n")
    f.write(f"   Shell: {default_shell_in_linux()}\n\n")
    f.write(f"   Uptime: {uptime_in_linux()}\n\n")
    
    

    f.close()

    



