class Xvn < Formula
  desc "Automatic Node.js version switching - 2-3x faster than avn"
  homepage "https://github.com/olvrcc/xvn"
  version "1.4.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/olvrcc/xvn/releases/download/v1.4.1/xvn-aarch64-apple-darwin.tar.gz"
      sha256 "27c643413774e73dd96eb892eb7d2ae62d74321da19e18a82c0d096e0204c7f3"
    else
      url "https://github.com/olvrcc/xvn/releases/download/v1.4.1/xvn-x86_64-apple-darwin.tar.gz"
      sha256 "6a207c38a7297cb91c583eb377d9262ce7ed7eaf5f73621dddec0d35a93c1e53"
    end
  end

  def install
    bin.install "xvn"
  end

  def caveats
    <<~EOS
      To set up xvn shell integration, run:
        xvn setup

      Then restart your shell or run:
        source ~/.bashrc  # or ~/.zshrc

      xvn requires a Node.js version manager (nvm or fnm) to be installed.
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/xvn --version")
  end
end
