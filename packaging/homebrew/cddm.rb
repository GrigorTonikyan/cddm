class Cddm < Formula
  desc "Polyglot Code De-Duplication Meister & Autonomous Refactoring Engine"
  homepage "https://git.gt-web-dev.com/gt-dev/cddm"
  version "1.9.0"
  license "MIT"

  on_macos do
    if Hardware::CPU.arm?
      url "https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v#{version}/cddm-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    else
      url "https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v#{version}/cddm-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v#{version}/cddm-v#{version}-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    else
      url "https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v#{version}/cddm-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "0000000000000000000000000000000000000000000000000000000000000000"
    end
  end

  def install
    bin.install "cddm"
    bin.install "cddm-mcp" if File.exist?("cddm-mcp")
  end

  test do
    assert_match "cddm", shell_output("#{bin}/cddm --version")
  end
end
