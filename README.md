This is a command-line script to visualize the commit history of a local git repository, similar to what GitHub displays on user profiles but for one repo only (which does not need to be hosted on GitHub) and using the command line instead of a web page.

# Installation

1. Install Rust:
    * On Windows, download and run [rustup-init.exe](https://win.rustup.rs/) and follow its instructions. If asked to install Visual C++ prerequisites, use the “Quick install via the Visual Studio Community installer” option. You can uncheck the option to launch Visual Studio when done.
    * On other platforms, please see [the Rust website](https://www.rust-lang.org/tools/install) for instructions.
2. Open a command line:
    * On Windows, right-click the start button, then click “Terminal”, “Windows PowerShell”, or “Command Prompt”.
    * On other platforms, look for an app named “Terminal” or similar.
3. In the command line, run the following command. Depending on your computer, this may take a while.

    ```
    cargo install --git=https://github.com/fenhl/git-vis --branch=main
    ```

# Usage

In a command line, run `git-vis path/to/repo`, replacing the placeholder path with the actual path to the git repository you'd like to visualize. If your working directory is in a git repository, the explicit path parameter can be omitted to use that repo.
