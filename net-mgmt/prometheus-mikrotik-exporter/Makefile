# $FreeBSD$

# vim: ts=8 noet

PORTNAME=		prometheus-mikrotik-exporter
PORTVERSION=		1.0.11
CATEGORIES=		net-mgmt

MAINTAINER=		kytomaki@iki.fi
COMMENT=		Mikrotik prometheus node-exporter

LICENSE=		BSD3CLAUSE

CONF_FILE=		mikrotik_exporter.yml

USES=			go

USE_RC_SUBR=		mikrotik_exporter

USE_GITHUB=		yes
GH_ACCOUNT=		nshttpd
GH_PROJECT=		mikrotik-exporter

GO_PKGNAME=		github.com/${GH_ACCOUNT}/${GH_PROJECT}

PLIST_FILES=		bin/mikrotik-exporter \
			"@sample etc/${CONF_FILE}.sample"
SUB_LIST+=		CONF_FILE=${CONF_FILE}


do-install:
		${INSTALL_PROGRAM} ${GO_WRKDIR_BIN}/mikrotik-exporter ${STAGEDIR}${PREFIX}/bin

post-install:
		${INSTALL_DATA} ${FILESDIR}/${CONF_FILE}.in ${STAGEDIR}${PREFIX}/etc/${CONF_FILE}.sample

.include <bsd.port.mk>
