class Cddm < Formula
  desc "Polyglot Code De-Duplication Meister & Autonomous Refactoring Engine"
  homepage "https://git.gt-web-dev.com/gt-dev/cddm"
  version "4.10.4"
  license "MIT"

  on_linux do
    on_intel do
      url "https://git.gt-web-dev.com/gt-dev/cddm/releases/download/v#{version}/cddm-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PENDING_RELEASE_SHA256"
    end
  end

  def install
    bin.install "cddm"
    bin.install "cddm-mcp" if File.exist?("cddm-mcp")
    bin.install "cddm-lsp" if File.exist?("cddm-lsp")

    generate_completions_from_executable(bin/"cddm", "completions")
  end

  test do
    assert_match "cddm", shell_output("#{bin}/cddm --version")
  end
end
