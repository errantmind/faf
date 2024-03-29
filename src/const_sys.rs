// This particular file is LICENSED AS GPLv2 (https://www.gnu.org/licenses/old-licenses/gpl-2.0.html)

#![allow(dead_code)]

// Manually transformed from /usr/include/x86_64-linux-gnu/asm/unistd_64.h

pub const SYS_READ: u32 = 0;
pub const SYS_WRITE: u32 = 1;
pub const SYS_OPEN: u32 = 2;
pub const SYS_CLOSE: u32 = 3;
pub const SYS_STAT: u32 = 4;
pub const SYS_FSTAT: u32 = 5;
pub const SYS_LSTAT: u32 = 6;
pub const SYS_POLL: u32 = 7;
pub const SYS_LSEEK: u32 = 8;
pub const SYS_MMAP: u32 = 9;
pub const SYS_MPROTECT: u32 = 10;
pub const SYS_MUNMAP: u32 = 11;
pub const SYS_BRK: u32 = 12;
pub const SYS_RT_SIGACTION: u32 = 13;
pub const SYS_RT_SIGPROCMASK: u32 = 14;
pub const SYS_RT_SIGRETURN: u32 = 15;
pub const SYS_IOCTL: u32 = 16;
pub const SYS_PREAD64: u32 = 17;
pub const SYS_PWRITE64: u32 = 18;
pub const SYS_READV: u32 = 19;
pub const SYS_WRITEV: u32 = 20;
pub const SYS_ACCESS: u32 = 21;
pub const SYS_PIPE: u32 = 22;
pub const SYS_SELECT: u32 = 23;
pub const SYS_SCHED_YIELD: u32 = 24;
pub const SYS_MREMAP: u32 = 25;
pub const SYS_MSYNC: u32 = 26;
pub const SYS_MINCORE: u32 = 27;
pub const SYS_MADVISE: u32 = 28;
pub const SYS_SHMGET: u32 = 29;
pub const SYS_SHMAT: u32 = 30;
pub const SYS_SHMCTL: u32 = 31;
pub const SYS_DUP: u32 = 32;
pub const SYS_DUP2: u32 = 33;
pub const SYS_PAUSE: u32 = 34;
pub const SYS_NANOSLEEP: u32 = 35;
pub const SYS_GETITIMER: u32 = 36;
pub const SYS_ALARM: u32 = 37;
pub const SYS_SETITIMER: u32 = 38;
pub const SYS_GETPID: u32 = 39;
pub const SYS_SENDFILE: u32 = 40;
pub const SYS_SOCKET: u32 = 41;
pub const SYS_CONNECT: u32 = 42;
pub const SYS_ACCEPT: u32 = 43;
pub const SYS_SENDTO: u32 = 44;
pub const SYS_RECVFROM: u32 = 45;
pub const SYS_SENDMSG: u32 = 46;
pub const SYS_RECVMSG: u32 = 47;
pub const SYS_SHUTDOWN: u32 = 48;
pub const SYS_BIND: u32 = 49;
pub const SYS_LISTEN: u32 = 50;
pub const SYS_GETSOCKNAME: u32 = 51;
pub const SYS_GETPEERNAME: u32 = 52;
pub const SYS_SOCKETPAIR: u32 = 53;
pub const SYS_SETSOCKOPT: u32 = 54;
pub const SYS_GETSOCKOPT: u32 = 55;
pub const SYS_CLONE: u32 = 56;
pub const SYS_FORK: u32 = 57;
pub const SYS_VFORK: u32 = 58;
pub const SYS_EXECVE: u32 = 59;
pub const SYS_EXIT: u32 = 60;
pub const SYS_WAIT4: u32 = 61;
pub const SYS_KILL: u32 = 62;
pub const SYS_UNAME: u32 = 63;
pub const SYS_SEMGET: u32 = 64;
pub const SYS_SEMOP: u32 = 65;
pub const SYS_SEMCTL: u32 = 66;
pub const SYS_SHMDT: u32 = 67;
pub const SYS_MSGGET: u32 = 68;
pub const SYS_MSGSND: u32 = 69;
pub const SYS_MSGRCV: u32 = 70;
pub const SYS_MSGCTL: u32 = 71;
pub const SYS_FCNTL: u32 = 72;
pub const SYS_FLOCK: u32 = 73;
pub const SYS_FSYNC: u32 = 74;
pub const SYS_FDATASYNC: u32 = 75;
pub const SYS_TRUNCATE: u32 = 76;
pub const SYS_FTRUNCATE: u32 = 77;
pub const SYS_GETDENTS: u32 = 78;
pub const SYS_GETCWD: u32 = 79;
pub const SYS_CHDIR: u32 = 80;
pub const SYS_FCHDIR: u32 = 81;
pub const SYS_RENAME: u32 = 82;
pub const SYS_MKDIR: u32 = 83;
pub const SYS_RMDIR: u32 = 84;
pub const SYS_CREAT: u32 = 85;
pub const SYS_LINK: u32 = 86;
pub const SYS_UNLINK: u32 = 87;
pub const SYS_SYMLINK: u32 = 88;
pub const SYS_READLINK: u32 = 89;
pub const SYS_CHMOD: u32 = 90;
pub const SYS_FCHMOD: u32 = 91;
pub const SYS_CHOWN: u32 = 92;
pub const SYS_FCHOWN: u32 = 93;
pub const SYS_LCHOWN: u32 = 94;
pub const SYS_UMASK: u32 = 95;
pub const SYS_GETTIMEOFDAY: u32 = 96;
pub const SYS_GETRLIMIT: u32 = 97;
pub const SYS_GETRUSAGE: u32 = 98;
pub const SYS_SYSINFO: u32 = 99;
pub const SYS_TIMES: u32 = 100;
pub const SYS_PTRACE: u32 = 101;
pub const SYS_GETUID: u32 = 102;
pub const SYS_SYSLOG: u32 = 103;
pub const SYS_GETGID: u32 = 104;
pub const SYS_SETUID: u32 = 105;
pub const SYS_SETGID: u32 = 106;
pub const SYS_GETEUID: u32 = 107;
pub const SYS_GETEGID: u32 = 108;
pub const SYS_SETPGID: u32 = 109;
pub const SYS_GETPPID: u32 = 110;
pub const SYS_GETPGRP: u32 = 111;
pub const SYS_SETSID: u32 = 112;
pub const SYS_SETREUID: u32 = 113;
pub const SYS_SETREGID: u32 = 114;
pub const SYS_GETGROUPS: u32 = 115;
pub const SYS_SETGROUPS: u32 = 116;
pub const SYS_SETRESUID: u32 = 117;
pub const SYS_GETRESUID: u32 = 118;
pub const SYS_SETRESGID: u32 = 119;
pub const SYS_GETRESGID: u32 = 120;
pub const SYS_GETPGID: u32 = 121;
pub const SYS_SETFSUID: u32 = 122;
pub const SYS_SETFSGID: u32 = 123;
pub const SYS_GETSID: u32 = 124;
pub const SYS_CAPGET: u32 = 125;
pub const SYS_CAPSET: u32 = 126;
pub const SYS_RT_SIGPENDING: u32 = 127;
pub const SYS_RT_SIGTIMEDWAIT: u32 = 128;
pub const SYS_RT_SIGQUEUEINFO: u32 = 129;
pub const SYS_RT_SIGSUSPEND: u32 = 130;
pub const SYS_SIGALTSTACK: u32 = 131;
pub const SYS_UTIME: u32 = 132;
pub const SYS_MKNOD: u32 = 133;
pub const SYS_USELIB: u32 = 134;
pub const SYS_PERSONALITY: u32 = 135;
pub const SYS_USTAT: u32 = 136;
pub const SYS_STATFS: u32 = 137;
pub const SYS_FSTATFS: u32 = 138;
pub const SYS_SYSFS: u32 = 139;
pub const SYS_GETPRIORITY: u32 = 140;
pub const SYS_SETPRIORITY: u32 = 141;
pub const SYS_SCHED_SETPARAM: u32 = 142;
pub const SYS_SCHED_GETPARAM: u32 = 143;
pub const SYS_SCHED_SETSCHEDULER: u32 = 144;
pub const SYS_SCHED_GETSCHEDULER: u32 = 145;
pub const SYS_SCHED_GET_PRIORITY_MAX: u32 = 146;
pub const SYS_SCHED_GET_PRIORITY_MIN: u32 = 147;
pub const SYS_SCHED_RR_GET_INTERVAL: u32 = 148;
pub const SYS_MLOCK: u32 = 149;
pub const SYS_MUNLOCK: u32 = 150;
pub const SYS_MLOCKALL: u32 = 151;
pub const SYS_MUNLOCKALL: u32 = 152;
pub const SYS_VHANGUP: u32 = 153;
pub const SYS_MODIFY_LDT: u32 = 154;
pub const SYS_PIVOT_ROOT: u32 = 155;
pub const SYS__SYSCTL: u32 = 156;
pub const SYS_PRCTL: u32 = 157;
pub const SYS_ARCH_PRCTL: u32 = 158;
pub const SYS_ADJTIMEX: u32 = 159;
pub const SYS_SETRLIMIT: u32 = 160;
pub const SYS_CHROOT: u32 = 161;
pub const SYS_SYNC: u32 = 162;
pub const SYS_ACCT: u32 = 163;
pub const SYS_SETTIMEOFDAY: u32 = 164;
pub const SYS_MOUNT: u32 = 165;
pub const SYS_UMOUNT2: u32 = 166;
pub const SYS_SWAPON: u32 = 167;
pub const SYS_SWAPOFF: u32 = 168;
pub const SYS_REBOOT: u32 = 169;
pub const SYS_SETHOSTNAME: u32 = 170;
pub const SYS_SETDOMAINNAME: u32 = 171;
pub const SYS_IOPL: u32 = 172;
pub const SYS_IOPERM: u32 = 173;
pub const SYS_CREATE_MODULE: u32 = 174;
pub const SYS_INIT_MODULE: u32 = 175;
pub const SYS_DELETE_MODULE: u32 = 176;
pub const SYS_GET_KERNEL_SYMS: u32 = 177;
pub const SYS_QUERY_MODULE: u32 = 178;
pub const SYS_QUOTACTL: u32 = 179;
pub const SYS_NFSSERVCTL: u32 = 180;
pub const SYS_GETPMSG: u32 = 181;
pub const SYS_PUTPMSG: u32 = 182;
pub const SYS_AFS_SYSCALL: u32 = 183;
pub const SYS_TUXCALL: u32 = 184;
pub const SYS_SECURITY: u32 = 185;
pub const SYS_GETTID: u32 = 186;
pub const SYS_READAHEAD: u32 = 187;
pub const SYS_SETXATTR: u32 = 188;
pub const SYS_LSETXATTR: u32 = 189;
pub const SYS_FSETXATTR: u32 = 190;
pub const SYS_GETXATTR: u32 = 191;
pub const SYS_LGETXATTR: u32 = 192;
pub const SYS_FGETXATTR: u32 = 193;
pub const SYS_LISTXATTR: u32 = 194;
pub const SYS_LLISTXATTR: u32 = 195;
pub const SYS_FLISTXATTR: u32 = 196;
pub const SYS_REMOVEXATTR: u32 = 197;
pub const SYS_LREMOVEXATTR: u32 = 198;
pub const SYS_FREMOVEXATTR: u32 = 199;
pub const SYS_TKILL: u32 = 200;
pub const SYS_TIME: u32 = 201;
pub const SYS_FUTEX: u32 = 202;
pub const SYS_SCHED_SETAFFINITY: u32 = 203;
pub const SYS_SCHED_GETAFFINITY: u32 = 204;
pub const SYS_SET_THREAD_AREA: u32 = 205;
pub const SYS_IO_SETUP: u32 = 206;
pub const SYS_IO_DESTROY: u32 = 207;
pub const SYS_IO_GETEVENTS: u32 = 208;
pub const SYS_IO_SUBMIT: u32 = 209;
pub const SYS_IO_CANCEL: u32 = 210;
pub const SYS_GET_THREAD_AREA: u32 = 211;
pub const SYS_LOOKUP_DCOOKIE: u32 = 212;
pub const SYS_EPOLL_CREATE: u32 = 213;
pub const SYS_EPOLL_CTL_OLD: u32 = 214;
pub const SYS_EPOLL_WAIT_OLD: u32 = 215;
pub const SYS_REMAP_FILE_PAGES: u32 = 216;
pub const SYS_GETDENTS64: u32 = 217;
pub const SYS_SET_TID_ADDRESS: u32 = 218;
pub const SYS_RESTART_SYSCALL: u32 = 219;
pub const SYS_SEMTIMEDOP: u32 = 220;
pub const SYS_FADVISE64: u32 = 221;
pub const SYS_TIMER_CREATE: u32 = 222;
pub const SYS_TIMER_SETTIME: u32 = 223;
pub const SYS_TIMER_GETTIME: u32 = 224;
pub const SYS_TIMER_GETOVERRUN: u32 = 225;
pub const SYS_TIMER_DELETE: u32 = 226;
pub const SYS_CLOCK_SETTIME: u32 = 227;
pub const SYS_CLOCK_GETTIME: u32 = 228;
pub const SYS_CLOCK_GETRES: u32 = 229;
pub const SYS_CLOCK_NANOSLEEP: u32 = 230;
pub const SYS_EXIT_GROUP: u32 = 231;
pub const SYS_EPOLL_WAIT: u32 = 232;
pub const SYS_EPOLL_CTL: u32 = 233;
pub const SYS_TGKILL: u32 = 234;
pub const SYS_UTIMES: u32 = 235;
pub const SYS_VSERVER: u32 = 236;
pub const SYS_MBIND: u32 = 237;
pub const SYS_SET_MEMPOLICY: u32 = 238;
pub const SYS_GET_MEMPOLICY: u32 = 239;
pub const SYS_MQ_OPEN: u32 = 240;
pub const SYS_MQ_UNLINK: u32 = 241;
pub const SYS_MQ_TIMEDSEND: u32 = 242;
pub const SYS_MQ_TIMEDRECEIVE: u32 = 243;
pub const SYS_MQ_NOTIFY: u32 = 244;
pub const SYS_MQ_GETSETATTR: u32 = 245;
pub const SYS_KEXEC_LOAD: u32 = 246;
pub const SYS_WAITID: u32 = 247;
pub const SYS_ADD_KEY: u32 = 248;
pub const SYS_REQUEST_KEY: u32 = 249;
pub const SYS_KEYCTL: u32 = 250;
pub const SYS_IOPRIO_SET: u32 = 251;
pub const SYS_IOPRIO_GET: u32 = 252;
pub const SYS_INOTIFY_INIT: u32 = 253;
pub const SYS_INOTIFY_ADD_WATCH: u32 = 254;
pub const SYS_INOTIFY_RM_WATCH: u32 = 255;
pub const SYS_MIGRATE_PAGES: u32 = 256;
pub const SYS_OPENAT: u32 = 257;
pub const SYS_MKDIRAT: u32 = 258;
pub const SYS_MKNODAT: u32 = 259;
pub const SYS_FCHOWNAT: u32 = 260;
pub const SYS_FUTIMESAT: u32 = 261;
pub const SYS_NEWFSTATAT: u32 = 262;
pub const SYS_UNLINKAT: u32 = 263;
pub const SYS_RENAMEAT: u32 = 264;
pub const SYS_LINKAT: u32 = 265;
pub const SYS_SYMLINKAT: u32 = 266;
pub const SYS_READLINKAT: u32 = 267;
pub const SYS_FCHMODAT: u32 = 268;
pub const SYS_FACCESSAT: u32 = 269;
pub const SYS_PSELECT6: u32 = 270;
pub const SYS_PPOLL: u32 = 271;
pub const SYS_UNSHARE: u32 = 272;
pub const SYS_SET_ROBUST_LIST: u32 = 273;
pub const SYS_GET_ROBUST_LIST: u32 = 274;
pub const SYS_SPLICE: u32 = 275;
pub const SYS_TEE: u32 = 276;
pub const SYS_SYNC_FILE_RANGE: u32 = 277;
pub const SYS_VMSPLICE: u32 = 278;
pub const SYS_MOVE_PAGES: u32 = 279;
pub const SYS_UTIMENSAT: u32 = 280;
pub const SYS_EPOLL_PWAIT: u32 = 281;
pub const SYS_SIGNALFD: u32 = 282;
pub const SYS_TIMERFD_CREATE: u32 = 283;
pub const SYS_EVENTFD: u32 = 284;
pub const SYS_FALLOCATE: u32 = 285;
pub const SYS_TIMERFD_SETTIME: u32 = 286;
pub const SYS_TIMERFD_GETTIME: u32 = 287;
pub const SYS_ACCEPT4: u32 = 288;
pub const SYS_SIGNALFD4: u32 = 289;
pub const SYS_EVENTFD2: u32 = 290;
pub const SYS_EPOLL_CREATE1: u32 = 291;
pub const SYS_DUP3: u32 = 292;
pub const SYS_PIPE2: u32 = 293;
pub const SYS_INOTIFY_INIT1: u32 = 294;
pub const SYS_PREADV: u32 = 295;
pub const SYS_PWRITEV: u32 = 296;
pub const SYS_RT_TGSIGQUEUEINFO: u32 = 297;
pub const SYS_PERF_EVENT_OPEN: u32 = 298;
pub const SYS_RECVMMSG: u32 = 299;
pub const SYS_FANOTIFY_INIT: u32 = 300;
pub const SYS_FANOTIFY_MARK: u32 = 301;
pub const SYS_PRLIMIT64: u32 = 302;
pub const SYS_NAME_TO_HANDLE_AT: u32 = 303;
pub const SYS_OPEN_BY_HANDLE_AT: u32 = 304;
pub const SYS_CLOCK_ADJTIME: u32 = 305;
pub const SYS_SYNCFS: u32 = 306;
pub const SYS_SENDMMSG: u32 = 307;
pub const SYS_SETNS: u32 = 308;
pub const SYS_GETCPU: u32 = 309;
pub const SYS_PROCESS_VM_READV: u32 = 310;
pub const SYS_PROCESS_VM_WRITEV: u32 = 311;
pub const SYS_KCMP: u32 = 312;
pub const SYS_FINIT_MODULE: u32 = 313;
pub const SYS_SCHED_SETATTR: u32 = 314;
pub const SYS_SCHED_GETATTR: u32 = 315;
pub const SYS_RENAMEAT2: u32 = 316;
pub const SYS_SECCOMP: u32 = 317;
pub const SYS_GETRANDOM: u32 = 318;
pub const SYS_MEMFD_CREATE: u32 = 319;
pub const SYS_KEXEC_FILE_LOAD: u32 = 320;
pub const SYS_BPF: u32 = 321;
pub const SYS_EXECVEAT: u32 = 322;
pub const SYS_USERFAULTFD: u32 = 323;
pub const SYS_MEMBARRIER: u32 = 324;
pub const SYS_MLOCK2: u32 = 325;
pub const SYS_COPY_FILE_RANGE: u32 = 326;
pub const SYS_PREADV2: u32 = 327;
pub const SYS_PWRITEV2: u32 = 328;
pub const SYS_PKEY_MPROTECT: u32 = 329;
pub const SYS_PKEY_ALLOC: u32 = 330;
pub const SYS_PKEY_FREE: u32 = 331;
pub const SYS_STATX: u32 = 332;
pub const SYS_IO_PGETEVENTS: u32 = 333;
pub const SYS_RSEQ: u32 = 334;
pub const SYS_PIDFD_SEND_SIGNAL: u32 = 424;
pub const SYS_IO_URING_SETUP: u32 = 425;
pub const SYS_IO_URING_ENTER: u32 = 426;
pub const SYS_IO_URING_REGISTER: u32 = 427;
pub const SYS_OPEN_TREE: u32 = 428;
pub const SYS_MOVE_MOUNT: u32 = 429;
pub const SYS_FSOPEN: u32 = 430;
pub const SYS_FSCONFIG: u32 = 431;
pub const SYS_FSMOUNT: u32 = 432;
pub const SYS_FSPICK: u32 = 433;
pub const SYS_PIDFD_OPEN: u32 = 434;
pub const SYS_CLONE3: u32 = 435;
pub const SYS_OPENAT2: u32 = 437;
pub const SYS_PIDFD_GETFD: u32 = 438;
pub const SYS_FACCESSAT2: u32 = 439;

// Manually transformed from /usr/include/asm-generic/errno-base.h
// and /usr/include/asm-generic/errno.h
pub const EPERM: u32 = 1;
pub const ENOENT: u32 = 2;
pub const ESRCH: u32 = 3;
pub const EINTR: u32 = 4;
pub const EIO: u32 = 5;
pub const ENXIO: u32 = 6;
pub const E2BIG: u32 = 7;
pub const ENOEXEC: u32 = 8;
pub const EBADF: u32 = 9;
pub const ECHILD: u32 = 10;
pub const EAGAIN: u32 = 11;
pub const ENOMEM: u32 = 12;
pub const EACCES: u32 = 13;
pub const EFAULT: u32 = 14;
pub const ENOTBLK: u32 = 15;
pub const EBUSY: u32 = 16;
pub const EEXIST: u32 = 17;
pub const EXDEV: u32 = 18;
pub const ENODEV: u32 = 19;
pub const ENOTDIR: u32 = 20;
pub const EISDIR: u32 = 21;
pub const EINVAL: u32 = 22;
pub const ENFILE: u32 = 23;
pub const EMFILE: u32 = 24;
pub const ENOTTY: u32 = 25;
pub const ETXTBSY: u32 = 26;
pub const EFBIG: u32 = 27;
pub const ENOSPC: u32 = 28;
pub const ESPIPE: u32 = 29;
pub const EROFS: u32 = 30;
pub const EMLINK: u32 = 31;
pub const EPIPE: u32 = 32;
pub const EDOM: u32 = 33;
pub const ERANGE: u32 = 34;
pub const EDEADLK: u32 = 35;
pub const ENAMETOOLONG: u32 = 36;
pub const ENOLCK: u32 = 37;
pub const ENOSYS: u32 = 38;
pub const ENOTEMPTY: u32 = 39;
pub const ELOOP: u32 = 40;
pub const EWOULDBLOCK: u32 = 11; // EAGAIN
pub const ENOMSG: u32 = 42;
pub const EIDRM: u32 = 43;
pub const ECHRNG: u32 = 44;
pub const EL2NSYNC: u32 = 45;
pub const EL3HLT: u32 = 46;
pub const EL3RST: u32 = 47;
pub const ELNRNG: u32 = 48;
pub const EUNATCH: u32 = 49;
pub const ENOCSI: u32 = 50;
pub const EL2HLT: u32 = 51;
pub const EBADE: u32 = 52;
pub const EBADR: u32 = 53;
pub const EXFULL: u32 = 54;
pub const ENOANO: u32 = 55;
pub const EBADRQC: u32 = 56;
pub const EBADSLT: u32 = 57;
pub const EDEADLOCK: u32 = 35; // EDEADLK
pub const EBFONT: u32 = 59;
pub const ENOSTR: u32 = 60;
pub const ENODATA: u32 = 61;
pub const ETIME: u32 = 62;
pub const ENOSR: u32 = 63;
pub const ENONET: u32 = 64;
pub const ENOPKG: u32 = 65;
pub const EREMOTE: u32 = 66;
pub const ENOLINK: u32 = 67;
pub const EADV: u32 = 68;
pub const ESRMNT: u32 = 69;
pub const ECOMM: u32 = 70;
pub const EPROTO: u32 = 71;
pub const EMULTIHOP: u32 = 72;
pub const EDOTDOT: u32 = 73;
pub const EBADMSG: u32 = 74;
pub const EOVERFLOW: u32 = 75;
pub const ENOTUNIQ: u32 = 76;
pub const EBADFD: u32 = 77;
pub const EREMCHG: u32 = 78;
pub const ELIBACC: u32 = 79;
pub const ELIBBAD: u32 = 80;
pub const ELIBSCN: u32 = 81;
pub const ELIBMAX: u32 = 82;
pub const ELIBEXEC: u32 = 83;
pub const EILSEQ: u32 = 84;
pub const ERESTART: u32 = 85;
pub const ESTRPIPE: u32 = 86;
pub const EUSERS: u32 = 87;
pub const ENOTSOCK: u32 = 88;
pub const EDESTADDRREQ: u32 = 89;
pub const EMSGSIZE: u32 = 90;
pub const EPROTOTYPE: u32 = 91;
pub const ENOPROTOOPT: u32 = 92;
pub const EPROTONOSUPPORT: u32 = 93;
pub const ESOCKTNOSUPPORT: u32 = 94;
pub const EOPNOTSUPP: u32 = 95;
pub const EPFNOSUPPORT: u32 = 96;
pub const EAFNOSUPPORT: u32 = 97;
pub const EADDRINUSE: u32 = 98;
pub const EADDRNOTAVAIL: u32 = 99;
pub const ENETDOWN: u32 = 100;
pub const ENETUNREACH: u32 = 101;
pub const ENETRESET: u32 = 102;
pub const ECONNABORTED: u32 = 103;
pub const ECONNRESET: u32 = 104;
pub const ENOBUFS: u32 = 105;
pub const EISCONN: u32 = 106;
pub const ENOTCONN: u32 = 107;
pub const ESHUTDOWN: u32 = 108;
pub const ETOOMANYREFS: u32 = 109;
pub const ETIMEDOUT: u32 = 110;
pub const ECONNREFUSED: u32 = 111;
pub const EHOSTDOWN: u32 = 112;
pub const EHOSTUNREACH: u32 = 113;
pub const EALREADY: u32 = 114;
pub const EINPROGRESS: u32 = 115;
pub const ESTALE: u32 = 116;
pub const EUCLEAN: u32 = 117;
pub const ENOTNAM: u32 = 118;
pub const ENAVAIL: u32 = 119;
pub const EISNAM: u32 = 120;
pub const EREMOTEIO: u32 = 121;
pub const EDQUOT: u32 = 122;
pub const ENOMEDIUM: u32 = 123;
pub const EMEDIUMTYPE: u32 = 124;
pub const ECANCELED: u32 = 125;
pub const ENOKEY: u32 = 126;
pub const EKEYEXPIRED: u32 = 127;
pub const EKEYREVOKED: u32 = 128;
pub const EKEYREJECTED: u32 = 129;
pub const EOWNERDEAD: u32 = 130;
pub const ENOTRECOVERABLE: u32 = 131;
pub const ERFKILL: u32 = 132;
pub const EHWPOISON: u32 = 133;

//

pub const O_DIRECT: i32 = 0x4000;
pub const O_DIRECTORY: i32 = 0x10000;
pub const O_NOFOLLOW: i32 = 0x20000;
pub const O_APPEND: i32 = 1024;
pub const O_CREAT: i32 = 64;
pub const O_EXCL: i32 = 128;
pub const O_NOCTTY: i32 = 256;
pub const O_NONBLOCK: i32 = 2048;
pub const O_SYNC: i32 = 1052672;
pub const O_RSYNC: i32 = 1052672;
pub const O_DSYNC: i32 = 4096;
pub const O_FSYNC: i32 = 0x101000;
pub const O_NOATIME: i32 = 0o1000000;
pub const O_PATH: i32 = 0o10000000;
pub const O_TMPFILE: i32 = 0o20000000 | O_DIRECTORY;

pub const SOL_SOCKET: i32 = 1;

pub const SO_REUSEADDR: i32 = 2;
pub const SO_TYPE: i32 = 3;
pub const SO_ERROR: i32 = 4;
pub const SO_DONTROUTE: i32 = 5;
pub const SO_BROADCAST: i32 = 6;
pub const SO_SNDBUF: i32 = 7;
pub const SO_RCVBUF: i32 = 8;
pub const SO_SNDBUFFORCE: i32 = 32;
pub const SO_RCVBUFFORCE: i32 = 33;
pub const SO_KEEPALIVE: i32 = 9;
pub const SO_OOBINLINE: i32 = 10;
pub const SO_NO_CHECK: i32 = 11;
pub const SO_PRIORITY: i32 = 12;
pub const SO_LINGER: i32 = 13;
pub const SO_BSDCOMPAT: i32 = 14;
pub const SO_REUSEPORT: i32 = 15;
pub const SO_PASSCRED: i32 = 16;
pub const SO_PEERCRED: i32 = 17;
pub const SO_RCVLOWAT: i32 = 18;
pub const SO_SNDLOWAT: i32 = 19;
pub const SO_RCVTIMEO: i32 = 20;
pub const SO_SNDTIMEO: i32 = 21;
pub const SO_SECURITY_AUTHENTICATION: i32 = 22;
pub const SO_SECURITY_ENCRYPTION_TRANSPORT: i32 = 23;
pub const SO_SECURITY_ENCRYPTION_NETWORK: i32 = 24;
pub const SO_BINDTODEVICE: i32 = 25;
pub const SO_ATTACH_FILTER: i32 = 26;
pub const SO_DETACH_FILTER: i32 = 27;
pub const SO_GET_FILTER: i32 = SO_ATTACH_FILTER;
pub const SO_PEERNAME: i32 = 28;
pub const SO_TIMESTAMP: i32 = 29;
pub const SO_ACCEPTCONN: i32 = 30;
pub const SO_PEERSEC: i32 = 31;
pub const SO_PASSSEC: i32 = 34;
pub const SO_TIMESTAMPNS: i32 = 35;
pub const SCM_TIMESTAMPNS: i32 = SO_TIMESTAMPNS;
pub const SO_MARK: i32 = 36;
pub const SO_PROTOCOL: i32 = 38;
pub const SO_DOMAIN: i32 = 39;
pub const SO_RXQ_OVFL: i32 = 40;
pub const SO_WIFI_STATUS: i32 = 41;
pub const SCM_WIFI_STATUS: i32 = SO_WIFI_STATUS;
pub const SO_PEEK_OFF: i32 = 42;
pub const SO_NOFCS: i32 = 43;
pub const SO_LOCK_FILTER: i32 = 44;
pub const SO_SELECT_ERR_QUEUE: i32 = 45;
pub const SO_BUSY_POLL: i32 = 46;
pub const SO_MAX_PACING_RATE: i32 = 47;
pub const SO_BPF_EXTENSIONS: i32 = 48;
pub const SO_INCOMING_CPU: i32 = 49;
pub const SO_ATTACH_BPF: i32 = 50;
pub const SO_DETACH_BPF: i32 = SO_DETACH_FILTER;
pub const SO_ATTACH_REUSEPORT_CBPF: i32 = 51;
pub const SO_ATTACH_REUSEPORT_EBPF: i32 = 52;
pub const SO_CNX_ADVICE: i32 = 53;
pub const SCM_TIMESTAMPING_OPT_STATS: i32 = 54;
pub const SO_MEMINFO: i32 = 55;
pub const SO_INCOMING_NAPI_ID: i32 = 56;
pub const SO_COOKIE: i32 = 57;
pub const SCM_TIMESTAMPING_PKTINFO: i32 = 58;
pub const SO_PEERGROUPS: i32 = 59;
pub const SO_ZEROCOPY: i32 = 60;
pub const SO_TXTIME: i32 = 61;
pub const SCM_TXTIME: i32 = SO_TXTIME;
pub const SO_BINDTOIFINDEX: i32 = 62;
pub const SO_TIMESTAMP_NEW: i32 = 63;
pub const SO_TIMESTAMPNS_NEW: i32 = 64;
pub const SO_TIMESTAMPING_NEW: i32 = 65;
pub const SO_RCVTIMEO_NEW: i32 = 66;
pub const SO_SNDTIMEO_NEW: i32 = 67;
pub const SO_DETACH_REUSEPORT_BPF: i32 = 68;

pub const SOCK_STREAM: i32 = 1;
pub const SOCK_DGRAM: i32 = 2;

pub const IPPROTO_TCP: i32 = 6;
pub const TCP_NODELAY: i32 = 1;
pub const TCP_MAXSEG: i32 = 2;
pub const TCP_CORK: i32 = 3;
pub const TCP_KEEPIDLE: i32 = 4;
pub const TCP_KEEPINTVL: i32 = 5;
pub const TCP_KEEPCNT: i32 = 6;
pub const TCP_SYNCNT: i32 = 7;
pub const TCP_LINGER2: i32 = 8;
pub const TCP_DEFER_ACCEPT: i32 = 9;
pub const TCP_WINDOW_CLAMP: i32 = 10;
pub const TCP_INFO: i32 = 11;
pub const TCP_QUICKACK: i32 = 12;
pub const TCP_CONGESTION: i32 = 13;
pub const TCP_COOKIE_TRANSACTIONS: i32 = 15;
pub const TCP_THIN_LINEAR_TIMEOUTS: i32 = 16;
pub const TCP_THIN_DUPACK: i32 = 17;
pub const TCP_USER_TIMEOUT: i32 = 18;
pub const TCP_REPAIR: i32 = 19;
pub const TCP_REPAIR_QUEUE: i32 = 20;
pub const TCP_QUEUE_SEQ: i32 = 21;
pub const TCP_REPAIR_OPTIONS: i32 = 22;
pub const TCP_FASTOPEN: i32 = 23;
pub const TCP_TIMESTAMP: i32 = 24;
pub const TCP_FASTOPEN_CONNECT: i32 = 30;

pub const INADDR_LOOPBACK: u32 = 2130706433;
pub const INADDR_ANY: u32 = 0;
pub const INADDR_BROADCAST: u32 = 4294967295;
pub const INADDR_NONE: u32 = 4294967295;

// EPOLL

pub const EPOLL_CTL_ADD: i32 = 1;
pub const EPOLL_CTL_MOD: i32 = 3;
pub const EPOLL_CTL_DEL: i32 = 2;

pub const EPOLLIN: u32 = 0x1;
pub const EPOLLPRI: u32 = 0x2;
pub const EPOLLOUT: u32 = 0x4;
pub const EPOLLRDNORM: u32 = 0x40;
pub const EPOLLRDBAND: u32 = 0x80;
pub const EPOLLWRNORM: u32 = 0x100;
pub const EPOLLWRBAND: u32 = 0x200;
pub const EPOLLMSG: u32 = 0x400;
pub const EPOLLERR: u32 = 0x8;
pub const EPOLLHUP: u32 = 0x10;
pub const EPOLLET: u32 = 0x80000000;
pub const EPOLLRDHUP: u32 = 0x2000;
pub const EPOLLEXCLUSIVE: u32 = 0x10000000;
pub const EPOLLONESHOT: u32 = 0x40000000;

// UNSHARE
pub const CLONE_FILES: u32 = 0x400;

// SET_PRIORITY
pub const PRIO_PROCESS: u32 = 0;
