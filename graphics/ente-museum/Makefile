PORTNAME=	ente-museum
PORTVERSION=	1.0.0.a.20240819
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
GH_TAGNAME=	cdbf8c5f0971cb383df03c6b2f72ffb85387beef

WRKSRC_SUBDIR=		server

GO_TARGET=		./cmd/museum:museum

# This is really clunky
post-extract:
	(cd ${WRKSRC}; ${GO_CMD} mod vendor)

test: ENV=test

.include <bsd.port.mk>
