# vim: ts=8 noet

PORTNAME=		prometheus-mikrotik-exporter
PORTVERSION=		1.0.12.a.2022616
CATEGORIES=		net-mgmt

MAINTAINER=		kytomaki@iki.fi
COMMENT=		Mikrotik prometheus node-exporter

LICENSE=		BSD3CLAUSE

USES=			go:modules
USE_GITHUB=		yes
GH_ACCOUNT=		nshttpd
GH_PROJECT=		mikrotik-exporter
GH_TAGNAME=		e1b06c6ebe6e71a5661326b3a33afe2fd741283d
GH_TUPLE=	\
		beorn7:perks:v1.0.1:beorn7_perks/vendor/github.com/beorn7/perks \
		cespare:xxhash:v2.1.1:cespare_xxhash_v2/vendor/github.com/cespare/xxhash/v2 \
		davecgh:go-spew:v1.1.1:davecgh_go_spew/vendor/github.com/davecgh/go-spew \
		go-routeros:routeros:1bbf141cdd91:go_routeros_routeros/vendor/gopkg.in/routeros.v2 \
		go-yaml:yaml:v2.4.0:go_yaml_yaml/vendor/gopkg.in/yaml.v2 \
		golang:net:aaa1db679c0d:golang_net/vendor/golang.org/x/net \
		golang:protobuf:v1.3.2:golang_protobuf/vendor/github.com/golang/protobuf \
		golang:sys:d867a43fc93e:golang_sys/vendor/golang.org/x/sys \
		matttproud:golang_protobuf_extensions:v1.0.1:matttproud_golang_protobuf_extensions/vendor/github.com/matttproud/golang_protobuf_extensions \
		miekg:dns:v1.1.43:miekg_dns/vendor/github.com/miekg/dns \
		pmezard:go-difflib:v1.0.0:pmezard_go_difflib/vendor/github.com/pmezard/go-difflib \
		prometheus:client_golang:v1.4.1:prometheus_client_golang/vendor/github.com/prometheus/client_golang \
		prometheus:client_model:v0.2.0:prometheus_client_model/vendor/github.com/prometheus/client_model \
		prometheus:common:v0.9.1:prometheus_common/vendor/github.com/prometheus/common \
		prometheus:procfs:v0.0.8:prometheus_procfs/vendor/github.com/prometheus/procfs \
		sirupsen:logrus:v1.8.1:sirupsen_logrus/vendor/github.com/sirupsen/logrus \
		stretchr:testify:v1.4.0:stretchr_testify/vendor/github.com/stretchr/testify

CONF_FILE=		mikrotik_exporter.yml

USE_RC_SUBR=		mikrotik_exporter

PLIST_FILES=		bin/prometheus-mikrotik-exporter \
			"@sample etc/${CONF_FILE}.sample"
SUB_LIST+=		CONF_FILE=${CONF_FILE}

do-install:
		${INSTALL_PROGRAM} ${GO_WRKDIR_BIN}/prometheus-mikrotik-exporter ${STAGEDIR}${PREFIX}/bin

post-install:
		${INSTALL_DATA} ${FILESDIR}/${CONF_FILE}.in ${STAGEDIR}${PREFIX}/etc/${CONF_FILE}.sample

.include <bsd.port.mk>
