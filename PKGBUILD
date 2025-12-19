# Maintainer: xRipzch
pkgname=tui-kanban
pkgver=0.4.0
pkgrel=1
pkgdesc="A simple, lightweight terminal-based kanban board built with Rust"
arch=('x86_64')
url="https://github.com/xRipzch/TUI-Kanban"
license=('MIT')
depends=('gcc-libs')
makedepends=('cargo' 'git')
source=("git+https://github.com/xRipzch/TUI-Kanban.git#tag=v${pkgver}")
sha256sums=('SKIP')

build() {
    cd "$srcdir/TUI-Kanban"
    cargo build --release --locked
}

check() {
    cd "$srcdir/TUI-Kanban"
    cargo test --release --locked
}

package() {
    cd "$srcdir/TUI-Kanban"
    install -Dm755 "target/release/tui-kanban" "$pkgdir/usr/bin/tui-kanban"
    install -Dm644 README.md "$pkgdir/usr/share/doc/tui-kanban/README.md"
}
