PORTNAME=	ente-museum
PORTVERSION=	1.0.0.a.20241120
CATEGORIES=	graphics

MAINTAINER=	kytomaki@iki.fi
COMMENT=	End to End Encrypted alternative to Google Photos and Apple Photos
WWW=		https://ente.io/

LICENSE=	AGPLv3

LIB_DEPENDS=	libsodium.so:security/libsodium

USES=		go:modules libtool pkgconfig
USE_GITHUB=	yes
GH_ACCOUNT=	ente-io
GH_PROJECT=	ente
# ente uses rolling tag `server/ghcr`
GH_TAGNAME=	b0674cd5f37cebda0f446db7ce1e87a3e6d8b9fc

WRKSRC_SUBDIR=		server

GO_TARGET=		./cmd/museum:museum

# This is really clunky
post-extract:
	(cd ${WRKSRC}; ${GO_CMD} mod vendor)

test: ENV=test

.include <bsd.port.mk>
