# vim: ts=8 noet

PORTNAME=	glyr
DISTVERSION=	1.0.10
CATEGORIES=	audio

MAINTAINER=	kytomaki@iki.fi
COMMENT=	Search engine for music related metadata
WWW=		https://github.com/sahib/glyr

LICENSE=	LGPL3

MY_DEPENDS=	glib>2:devel/glib20 \
		curl>0:ftp/curl
LIB_DEPENDS= 	libcurl.so:ftp/curl \
		libglib-2.0.so:devel/glib20
BUILD_DEPENDS=	${MY_DEPENDS}

USES=		cmake ninja sqlite gettext-runtime pkgconfig
USE_LDCONFIG=	yes
USE_GCC=	yes

USE_GITHUB=	yes
GH_ACCOUNT=	sahib
GH_PROJECT=	glyr

.include <bsd.port.mk>
