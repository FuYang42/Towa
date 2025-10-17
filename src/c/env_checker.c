#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifdef _WIN32
    #include <windows.h>
    #define OS_NAME "Windows"
    #define PATH_SEP ";"
#else
    #include <unistd.h>
    #include <sys/utsname.h>
    #define OS_NAME "Unix"
    #define PATH_SEP ":"
#endif

// 颜色输出宏（Windows 和 Unix）
#ifdef _WIN32
    #define COLOR_GREEN ""
    #define COLOR_RED ""
    #define COLOR_YELLOW ""
    #define COLOR_RESET ""
#else
    #define COLOR_GREEN "\033[0;32m"
    #define COLOR_RED "\033[0;31m"
    #define COLOR_YELLOW "\033[0;33m"
    #define COLOR_RESET "\033[0m"
#endif

typedef struct {
    const char* name;
    const char* command;
    int found;
    char version[256];
} Tool;

// 检查命令是否存在
int check_command(const char* cmd, char* version_output, size_t size) {
    char command[512];
    FILE* fp;

#ifdef _WIN32
    snprintf(command, sizeof(command), "%s --version 2>nul", cmd);
#else
    snprintf(command, sizeof(command), "%s --version 2>/dev/null", cmd);
#endif

    fp = popen(command, "r");
    if (fp == NULL) {
        return 0;
    }

    if (fgets(version_output, size, fp) != NULL) {
        // 移除换行符
        version_output[strcspn(version_output, "\n")] = 0;
        pclose(fp);
        return 1;
    }

    pclose(fp);
    return 0;
}

// 打印系统信息
void print_system_info() {
    printf("\n=== System Information ===\n");

#ifdef _WIN32
    OSVERSIONINFOEX osvi;
    ZeroMemory(&osvi, sizeof(OSVERSIONINFOEX));
    osvi.dwOSVersionInfoSize = sizeof(OSVERSIONINFOEX);

    printf("OS: Windows\n");

    // 获取计算机名
    char computer_name[256];
    DWORD size = sizeof(computer_name);
    if (GetComputerNameA(computer_name, &size)) {
        printf("Computer: %s\n", computer_name);
    }
#else
    struct utsname buffer;
    if (uname(&buffer) == 0) {
        printf("OS: %s\n", buffer.sysname);
        printf("Version: %s\n", buffer.release);
        printf("Architecture: %s\n", buffer.machine);
        printf("Hostname: %s\n", buffer.nodename);
    }
#endif
}

// 检查开发工具
void check_dev_tools() {
    printf("\n=== Development Tools ===\n");

    Tool tools[] = {
        {"GCC", "gcc", 0, ""},
        {"Clang", "clang", 0, ""},
        {"CMake", "cmake", 0, ""},
        {"Make", "make", 0, ""},
        {"Git", "git", 0, ""},
        {"Rust", "rustc", 0, ""},
        {"Cargo", "cargo", 0, ""},
        {"Python", "python", 0, ""},
        {"Node.js", "node", 0, ""}
    };

    int num_tools = sizeof(tools) / sizeof(Tool);

    for (int i = 0; i < num_tools; i++) {
        tools[i].found = check_command(tools[i].command, tools[i].version, sizeof(tools[i].version));

        if (tools[i].found) {
            printf("%s[✓]%s %-10s: %s\n", COLOR_GREEN, COLOR_RESET, tools[i].name, tools[i].version);
        } else {
            printf("%s[✗]%s %-10s: Not found\n", COLOR_RED, COLOR_RESET, tools[i].name);
        }
    }
}

// 检查环境变量
void check_environment() {
    printf("\n=== Environment Variables ===\n");

    const char* vars[] = {
        "PATH",
        "CEPTON_SDK_PATH",
        "CARGO_HOME",
        "RUSTUP_HOME"
    };

    int num_vars = sizeof(vars) / sizeof(char*);

    for (int i = 0; i < num_vars; i++) {
        char* value = getenv(vars[i]);
        if (value != NULL) {
            if (strcmp(vars[i], "PATH") == 0) {
                printf("%s[✓]%s %s: (set, %zu chars)\n", COLOR_GREEN, COLOR_RESET, vars[i], strlen(value));
            } else {
                printf("%s[✓]%s %s: %s\n", COLOR_GREEN, COLOR_RESET, vars[i], value);
            }
        } else {
            printf("%s[✗]%s %s: Not set\n", COLOR_YELLOW, COLOR_RESET, vars[i]);
        }
    }
}

// 生成建议
void print_recommendations() {
    printf("\n=== Recommendations ===\n");

    char version[256];

    if (!check_command("gcc", version, sizeof(version)) &&
        !check_command("clang", version, sizeof(version))) {
        printf("%s[!]%s Install a C compiler (GCC or Clang)\n", COLOR_YELLOW, COLOR_RESET);
    }

    if (!check_command("cmake", version, sizeof(version))) {
        printf("%s[!]%s Install CMake for build configuration\n", COLOR_YELLOW, COLOR_RESET);
    }

    if (!check_command("git", version, sizeof(version))) {
        printf("%s[!]%s Install Git for version control\n", COLOR_YELLOW, COLOR_RESET);
    }

    if (!check_command("rustc", version, sizeof(version))) {
        printf("%s[!]%s Install Rust toolchain via rustup\n", COLOR_YELLOW, COLOR_RESET);
    }

    if (getenv("CEPTON_SDK_PATH") == NULL) {
        printf("%s[!]%s Set CEPTON_SDK_PATH environment variable\n", COLOR_YELLOW, COLOR_RESET);
    }
}

int main(int argc, char* argv[]) {
    printf("╔════════════════════════════════════════╗\n");
    printf("║   Towa Environment Checker v1.0        ║\n");
    printf("║   Development Environment Scanner       ║\n");
    printf("╚════════════════════════════════════════╝\n");

    print_system_info();
    check_dev_tools();
    check_environment();
    print_recommendations();

    printf("\n");
    return 0;
}
