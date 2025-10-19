class Anvs < Formula
  desc "Automatic Node.js version switching - fast, modern CLI"
  homepage "https://github.com/olvrcc/anvs"
  version "2.1.1"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/olvrcc/anvs/releases/download/v2.1.1/anvs-aarch64-apple-darwin.tar.gz"
      sha256 "c6eb370de80ac6437ac6a275400b9670fe710f8ef55a0dd733dda5a1393b7ca7"
    else
      url "https://github.com/olvrcc/anvs/releases/download/v2.1.1/anvs-x86_64-apple-darwin.tar.gz"
      sha256 "32afcdc2092ace2b288b335324a48d209a7e72c850123eeacbc7dcc2db7119ea"
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