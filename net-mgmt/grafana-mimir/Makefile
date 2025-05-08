# vim: ts=8 noet

PORTNAME=		grafana-mimir
PORTVERSION=		2.12.0
DISTVERSIONPREFIX=	mimir-
CATEGORIES=		net-mgmt

MAINTAINER=		kytomaki@iki.fi
COMMENT=		Grafana Mimir

LICENSE=		AGPLv3

USES=			go:modules
USE_GITHUB=		yes
GH_ACCOUNT=		grafana
GH_PROJECT=		mimir

GO_TARGET=		./cmd/mimir ./cmd/metaconvert ./cmd/mimirtool ./cmd/query-tee

USE_RC_SUBR=		grafana_mimir
CONF_FILE=		grafana_mimir.yml

PLIST_FILES=		bin/mimir \
			bin/metaconvert \
			bin/mimirtool \
			bin/query-tee \
			"@sample etc/${CONF_FILE}.sample" \
			"@dir /var/db/mimir"
SUB_LIST+=		CONF_FILE=${CONF_FILE}

do-install:
		${INSTALL_PROGRAM} ${GO_WRKDIR_BIN}/mimir ${STAGEDIR}${PREFIX}/bin
		${INSTALL_PROGRAM} ${GO_WRKDIR_BIN}/metaconvert ${STAGEDIR}${PREFIX}/bin
		${INSTALL_PROGRAM} ${GO_WRKDIR_BIN}/mimirtool ${STAGEDIR}${PREFIX}/bin
		${INSTALL_PROGRAM} ${GO_WRKDIR_BIN}/query-tee ${STAGEDIR}${PREFIX}/bin

post-install:
		${INSTALL_DATA} ${FILESDIR}/${CONF_FILE}.in ${STAGEDIR}${PREFIX}/etc/${CONF_FILE}.sample
		${MKDIR} ${STAGEDIR}/var/db/mimir

.include <bsd.port.mk>
