class Anvs < Formula
  desc "Automatic Node.js version switching - fast, modern CLI"
  homepage "https://github.com/olvrcc/anvs"
  version "2.1.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/olvrcc/anvs/releases/download/v2.1.0/anvs-aarch64-apple-darwin.tar.gz"
      sha256 "592f45e735382cb80c7fe6d429754cd4f913f4d1b2a8bcfc26ab9fa2823392ad"
    else
      url "https://github.com/olvrcc/anvs/releases/download/v2.1.0/anvs-x86_64-apple-darwin.tar.gz"
      sha256 "533d754c52bdaa20607dfa80affaade5e851c8a24868325880ff38def604bb17"
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