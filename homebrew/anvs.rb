class Anvs < Formula
  desc "Automatic Node.js version switching - fast, modern CLI"
  homepage "https://github.com/olvrcc/anvs"
  version "2.1.2"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/olvrcc/anvs/releases/download/v2.1.2/anvs-aarch64-apple-darwin.tar.gz"
      sha256 "b3f707780aa757e4bae89b36fc742264d3ab8e8bc3b72e89326133a33584b3bf"
    else
      url "https://github.com/olvrcc/anvs/releases/download/v2.1.2/anvs-x86_64-apple-darwin.tar.gz"
      sha256 "b3ef8fbc8042549016c0f174d2110ed45734b8f22b9517a61eeed2fde23da9e4"
    end
  end

  def install
    bin.install "anvs"
  end

  def caveats
    <<~EOS
      To set up anvs shell integration, run:
        anvs init

      Then restart your shell or run:
        source ~/.bashrc  # or ~/.zshrc

      anvs requires a Node.js version manager (nvm or fnm) to be installed.
    EOS
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/anvs --version")
  end
end