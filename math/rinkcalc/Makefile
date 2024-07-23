PORTNAME=		rinkcalc
DISTVERSIONPREFIX=	v
DISTVERSION=		0.8.0
CATEGORIES=		math
WWW=			https://rinkcalc.app/about

MAINTAINER=		kytomaki@iki.fi
COMMENT=		Unit conversion tool and library

LICENSE=		MPL20

USES=			cargo ssl gmake
USE_GITHUB=		yes
GH_ACCOUNT=		tiffany352
GH_PROJECT=		rink-rs

BUILD_DEPENDS=		asciidoctor:textproc/rubygem-asciidoctor
LIB_DEPENDS+=		libcurl.so:ftp/curl
CARGO_BUILD=		no
CARGO_INSTALL=		no
CARGO_CARGOTOML=	${WRKSRC}/cli/Cargo.toml

.include <bsd.port.mk>
