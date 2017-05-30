#include <string.h>
#include <libproc.h>


int32_t macprocinfo_getppid(const int32_t pid)
{
    struct proc_bsdinfo info;
    proc_pidinfo((pid_t)pid, PROC_PIDTBSDINFO, 0, &info, sizeof(info));
    return (int32_t)info.pbi_ppid;
}

int macprocinfo_getpidpath(const int32_t pid, char *buf, uint32_t bufsize)
{
    int rv = proc_pidpath((pid_t)pid, buf, (size_t)bufsize);
    if (rv <= 0) {
        return 0;
    } else {
        return 1;
    }
}

int macprocinfo_getpidname(const int32_t pid, char *buf, uint32_t bufsize)
{
    int rv = proc_name((pid_t)pid, buf, (size_t)bufsize);
    if (rv <= 0) {
        return 0;
    } else {
        return 1;
    }
}
