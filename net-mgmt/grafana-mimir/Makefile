# $FreeBSD$

# vim: ts=8 noet

PORTNAME=		grafana-mimir
PORTVERSION=		2.12.0
DISTVERSIONPREFIX=	mimir-
CATEGORIES=		net-mgmt

MAINTAINER=		kytomaki@iki.fi
COMMENT=		Grafana Mimir

LICENSE=		AGPLv3

CONF_FILE=		grafana_mimir.yml

USES=			go:modules

GO_TARGET=	./cmd/mimir ./cmd/metaconvert ./cmd/mimirtool ./cmd/query-tee

USE_RC_SUBR=		grafana_mimir

USE_GITHUB=		yes
GH_ACCOUNT=		grafana
GH_PROJECT=		mimir
GH_TUPLE=	\
		Azure:azure-sdk-for-go:0270c003ff8f:azure_azure_sdk_for_go_internal/vendor/github.com/Azure/azure-sdk-for-go \
		Azure:azure-sdk-for-go:58ac9ec45a48:azure_azure_sdk_for_go_azblob \
		Azure:azure-sdk-for-go:bfa13229abec:azure_azure_sdk_for_go_azcore \
		Azure:azure-sdk-for-go:defa37ddface:azure_azure_sdk_for_go_azidentity \
		AzureAD:microsoft-authentication-library-for-go:v1.2.1:azuread_microsoft_authentication_library_for_go/vendor/github.com/AzureAD/microsoft-authentication-library-for-go \
		DmitriyVTitov:size:v1.5.0:dmitriyvtitov_size/vendor/github.com/DmitriyVTitov/size \
		HdrHistogram:hdrhistogram-go:v1.1.2:hdrhistogram_hdrhistogram_go/vendor/github.com/HdrHistogram/hdrhistogram-go \
		alecthomas:chroma:v2.12.0:alecthomas_chroma_v2/vendor/github.com/alecthomas/chroma/v2 \
		alecthomas:kingpin:v2.4.0:alecthomas_kingpin_v2/vendor/github.com/alecthomas/kingpin/v2 \
		alecthomas:units:9a357b53e9c9:alecthomas_units/vendor/github.com/alecthomas/units \
		armon:go-metrics:v0.4.1:armon_go_metrics/vendor/github.com/armon/go-metrics \
		asaskevich:govalidator:a9d515a09cc2:asaskevich_govalidator/vendor/github.com/asaskevich/govalidator \
		aws:aws-sdk-go-v2:v1.11.0:aws_aws_sdk_go_v2_credentials \
		aws:aws-sdk-go-v2:v1.11.1:aws_aws_sdk_go_v2_sso \
		aws:aws-sdk-go-v2:v1.16.0:aws_aws_sdk_go_v2 \
		aws:aws-sdk-go-v2:v1.16.1:aws_aws_sdk_go_v2_sts \
		aws:aws-sdk-go-v2:v1.9.1:aws_aws_sdk_go_v2_presigned_url/vendor/github.com/aws/aws-sdk-go-v2 \
		aws:aws-sdk-go:v1.50.25:aws_aws_sdk_go/vendor/github.com/aws/aws-sdk-go \
		aws:smithy-go:v1.11.1:aws_smithy_go/vendor/github.com/aws/smithy-go \
		bboreham:go-loser:fcc2c21820a3:bboreham_go_loser/vendor/github.com/bboreham/go-loser \
		benbjohnson:clock:v1.3.5:benbjohnson_clock/vendor/github.com/benbjohnson/clock \
		beorn7:perks:v1.0.1:beorn7_perks/vendor/github.com/beorn7/perks \
		bits-and-blooms:bitset:v1.8.0:bits_and_blooms_bitset/vendor/github.com/bits-and-blooms/bitset \
		cenkalti:backoff:v3.2.2:cenkalti_backoff_v3/vendor/github.com/cenkalti/backoff/v3 \
		cenkalti:backoff:v4.2.1:cenkalti_backoff_v4/vendor/github.com/cenkalti/backoff/v4 \
		census-instrumentation:opencensus-go:v0.24.0:census_instrumentation_opencensus_go/vendor/go.opencensus.io \
		cespare:xxhash:v1.1.0:cespare_xxhash/vendor/github.com/cespare/xxhash \
		cespare:xxhash:v2.2.0:cespare_xxhash_v2/vendor/github.com/cespare/xxhash/v2 \
		charleskorn:go-grpc:e9298576254f:charleskorn_go_grpc/vendor/github.com/opentracing-contrib/go-grpc \
		colega:go-yaml-yaml:255a8d16d094:colega_go_yaml_yaml/vendor/gopkg.in/yaml.v3 \
		coreos:go-semver:v0.3.0:coreos_go_semver/vendor/github.com/coreos/go-semver \
		coreos:go-systemd:v22.5.0:coreos_go_systemd_v22/vendor/github.com/coreos/go-systemd/v22 \
		davecgh:go-spew:d8f796af33cc:davecgh_go_spew/vendor/github.com/davecgh/go-spew \
		dennwc:varint:v1.0.0:dennwc_varint/vendor/github.com/dennwc/varint \
		dgraph-io:ristretto:v0.1.1:dgraph_io_ristretto/vendor/github.com/dgraph-io/ristretto \
		dgryski:go-rendezvous:9f7001d12a5f:dgryski_go_rendezvous/vendor/github.com/dgryski/go-rendezvous \
		dlclark:regexp2:v1.10.0:dlclark_regexp2/vendor/github.com/dlclark/regexp2 \
		docker:go-connections:58542c764a11:docker_go_connections/vendor/github.com/docker/go-connections \
		docker:go-units:v0.5.0:docker_go_units/vendor/github.com/docker/go-units \
		dustin:go-humanize:v1.0.1:dustin_go_humanize/vendor/github.com/dustin/go-humanize \
		edsrzf:mmap-go:v1.1.0:edsrzf_mmap_go/vendor/github.com/edsrzf/mmap-go \
		efficientgo:core:ba593f67d2a4:efficientgo_core/vendor/github.com/efficientgo/core \
		efficientgo:e2e:8fa9daa8af8a:efficientgo_e2e/vendor/github.com/efficientgo/e2e \
		etcd-io:etcd:v3.5.4:etcd_io_etcd/vendor/go.etcd.io/etcd/api/v3 \
		etcd-io:etcd:v3.5.4:etcd_io_etcd_1/vendor/go.etcd.io/etcd/client/pkg/v3 \
		etcd-io:etcd:v3.5.4:etcd_io_etcd_2/vendor/go.etcd.io/etcd/client/v3 \
		facette:natsort:2cd4dd1e2dcb:facette_natsort/vendor/github.com/facette/natsort \
		failsafe-go:failsafe-go:v0.4.4:failsafe_go_failsafe_go/vendor/github.com/failsafe-go/failsafe-go \
		fatih:color:v1.15.0:fatih_color/vendor/github.com/fatih/color \
		felixge:fgprof:v0.9.3:felixge_fgprof/vendor/github.com/felixge/fgprof \
		felixge:httpsnoop:v1.0.4:felixge_httpsnoop/vendor/github.com/felixge/httpsnoop \
		fsnotify:fsnotify:v1.7.0:fsnotify_fsnotify/vendor/github.com/fsnotify/fsnotify \
		go-errors:errors:v1.4.2:go_errors_errors/vendor/github.com/go-errors/errors \
		go-ini:ini:v1.67.0:go_ini_ini/vendor/gopkg.in/ini.v1 \
		go-jose:go-jose:v3.0.3:go_jose_go_jose_v3/vendor/github.com/go-jose/go-jose/v3 \
		go-kit:log:v0.2.1:go_kit_log/vendor/github.com/go-kit/log \
		go-logfmt:logfmt:v0.6.0:go_logfmt_logfmt/vendor/github.com/go-logfmt/logfmt \
		go-logr:logr:v1.4.1:go_logr_logr/vendor/github.com/go-logr/logr \
		go-logr:stdr:v1.2.2:go_logr_stdr/vendor/github.com/go-logr/stdr \
		go-openapi:analysis:v0.22.2:go_openapi_analysis/vendor/github.com/go-openapi/analysis \
		go-openapi:errors:v0.21.0:go_openapi_errors/vendor/github.com/go-openapi/errors \
		go-openapi:jsonpointer:v0.20.2:go_openapi_jsonpointer/vendor/github.com/go-openapi/jsonpointer \
		go-openapi:jsonreference:v0.20.4:go_openapi_jsonreference/vendor/github.com/go-openapi/jsonreference \
		go-openapi:loads:v0.21.5:go_openapi_loads/vendor/github.com/go-openapi/loads \
		go-openapi:runtime:v0.27.1:go_openapi_runtime/vendor/github.com/go-openapi/runtime \
		go-openapi:spec:v0.20.14:go_openapi_spec/vendor/github.com/go-openapi/spec \
		go-openapi:strfmt:v0.22.0:go_openapi_strfmt/vendor/github.com/go-openapi/strfmt \
		go-openapi:swag:v0.22.9:go_openapi_swag/vendor/github.com/go-openapi/swag \
		go-openapi:validate:v0.23.0:go_openapi_validate/vendor/github.com/go-openapi/validate \
		go-redis:redis:v8.11.5:go_redis_redis_v8/vendor/github.com/go-redis/redis/v8 \
		go-telebot:telebot:v3.2.1:go_telebot_telebot/vendor/gopkg.in/telebot.v3 \
		go-test:deep:v1.1.0:go_test_deep/vendor/github.com/go-test/deep \
		go-yaml:yaml:v2.4.0:go_yaml_yaml/vendor/gopkg.in/yaml.v2 \
		gofrs:uuid:v4.4.0:gofrs_uuid/vendor/github.com/gofrs/uuid \
		gogo:googleapis:v1.4.1:gogo_googleapis/vendor/github.com/gogo/googleapis \
		gogo:protobuf:v1.3.2:gogo_protobuf/vendor/github.com/gogo/protobuf \
		gogo:status:v1.1.1:gogo_status/vendor/github.com/gogo/status \
		golang-jwt:jwt:v5.2.0:golang_jwt_jwt_v5/vendor/github.com/golang-jwt/jwt/v5 \
		golang:appengine:v1.6.8:golang_appengine/vendor/google.golang.org/appengine \
		golang:crypto:v0.19.0:golang_crypto/vendor/golang.org/x/crypto \
		golang:exp:814bf88cf225:golang_exp/vendor/golang.org/x/exp \
		golang:glog:v1.1.2:golang_glog/vendor/github.com/golang/glog \
		golang:groupcache:41bb18bfe9da:golang_groupcache/vendor/github.com/golang/groupcache \
		golang:mod:v0.15.0:golang_mod/vendor/golang.org/x/mod \
		golang:net:v0.21.0:golang_net/vendor/golang.org/x/net \
		golang:oauth2:v0.16.0:golang_oauth2/vendor/golang.org/x/oauth2 \
		golang:protobuf:v1.5.3:golang_protobuf/vendor/github.com/golang/protobuf \
		golang:snappy:v0.0.4:golang_snappy/vendor/github.com/golang/snappy \
		golang:sync:v0.6.0:golang_sync/vendor/golang.org/x/sync \
		golang:sys:v0.17.0:golang_sys/vendor/golang.org/x/sys \
		golang:text:v0.14.0:golang_text/vendor/golang.org/x/text \
		golang:time:v0.5.0:golang_time/vendor/golang.org/x/time \
		golang:tools:v0.18.0:golang_tools/vendor/golang.org/x/tools \
		google:btree:v1.0.1:google_btree/vendor/github.com/google/btree \
		google:gnostic-models:v0.6.8:google_gnostic_models/vendor/github.com/google/gnostic-models \
		google:go-cmp:v0.6.0:google_go_cmp/vendor/github.com/google/go-cmp \
		google:go-genproto:012b6fc9bca9:google_go_genproto_1/vendor/google.golang.org/genproto/googleapis/rpc \
		google:go-genproto:31a09d347014:google_go_genproto/vendor/google.golang.org/genproto \
		google:go-github:v57.0.0:google_go_github_v57/vendor/github.com/google/go-github/v57 \
		google:go-querystring:v1.1.0:google_go_querystring/vendor/github.com/google/go-querystring \
		google:gopacket:v1.1.19:google_gopacket/vendor/github.com/google/gopacket \
		google:pprof:35fc243c5815:google_pprof/vendor/github.com/google/pprof \
		google:s2a-go:v0.1.7:google_s2a_go/vendor/github.com/google/s2a-go \
		google:uuid:v1.6.0:google_uuid/vendor/github.com/google/uuid \
		googleapis:enterprise-certificate-proxy:v0.3.2:googleapis_enterprise_certificate_proxy/vendor/github.com/googleapis/enterprise-certificate-proxy \
		googleapis:gax-go:v2.12.0:googleapis_gax_go_v2/vendor/github.com/googleapis/gax-go \
		googleapis:google-api-go-client:v0.162.0:googleapis_google_api_go_client/vendor/google.golang.org/api \
		googleapis:google-cloud-go:compute/v1.23.4:googleapis_google_cloud_go_compute \
		googleapis:google-cloud-go:iam/v1.1.6:googleapis_google_cloud_go_iam \
		googleapis:google-cloud-go:storage/v1.38.0:googleapis_google_cloud_go_storage \
		googleapis:google-cloud-go:v0.112.0:googleapis_google_cloud_go/vendor/cloud.google.com/go \
		googleapis:google-cloud-go:compute/metadata/v0.2.3:googleapis_google_cloud_go_1/vendor/cloud.google.com/go/compute/metadata \
		gorilla:mux:v1.8.1:gorilla_mux/vendor/github.com/gorilla/mux \
		gosimple:slug:v1.1.1:gosimple_slug/vendor/github.com/gosimple/slug \
		grafana-tools:sdk:6562121319fc:grafana_tools_sdk/vendor/github.com/grafana-tools/sdk \
		grafana:dskit:b073df3747b2:grafana_dskit/vendor/github.com/grafana/dskit \
		grafana:e2e:db90b84177fc:grafana_e2e/vendor/github.com/grafana/e2e \
		grafana:goautoneg:47ce5e72a9ae:grafana_goautoneg/vendor/github.com/munnerz/goautoneg \
		grafana:gomemcache:cd6a66d6fb56:grafana_gomemcache/vendor/github.com/grafana/gomemcache \
		grafana:memberlist:09ffed8adbbe:grafana_memberlist/vendor/github.com/hashicorp/memberlist \
		grafana:mimir-prometheus:f1f7b4f2e5dc:grafana_mimir_prometheus/vendor/github.com/prometheus/prometheus \
		grafana:opentracing-contrib-go-stdlib:f410e79da956:grafana_opentracing_contrib_go_stdlib/vendor/github.com/opentracing-contrib/go-stdlib \
		grafana:pyroscope-go:godeltaprof/v0.1.6:grafana_pyroscope_go_godeltaprof/vendor/github.com/grafana/pyroscope-go \
		grafana:regexp:b4c2bcb0a4b6:grafana_regexp/vendor/github.com/grafana/regexp \
		grpc:grpc-go:v1.61.1:grpc_grpc_go/vendor/google.golang.org/grpc \
		hashicorp:consul:api/v1.27.0:hashicorp_consul_api/vendor/github.com/hashicorp/consul \
		hashicorp:errwrap:v1.1.0:hashicorp_errwrap/vendor/github.com/hashicorp/errwrap \
		hashicorp:go-cleanhttp:v0.5.2:hashicorp_go_cleanhttp/vendor/github.com/hashicorp/go-cleanhttp \
		hashicorp:go-hclog:v1.5.0:hashicorp_go_hclog/vendor/github.com/hashicorp/go-hclog \
		hashicorp:go-immutable-radix:v1.3.1:hashicorp_go_immutable_radix/vendor/github.com/hashicorp/go-immutable-radix \
		hashicorp:go-msgpack:v1.1.5:hashicorp_go_msgpack/vendor/github.com/hashicorp/go-msgpack \
		hashicorp:go-multierror:v1.1.1:hashicorp_go_multierror/vendor/github.com/hashicorp/go-multierror \
		hashicorp:go-retryablehttp:v0.7.4:hashicorp_go_retryablehttp/vendor/github.com/hashicorp/go-retryablehttp \
		hashicorp:go-rootcerts:v1.0.2:hashicorp_go_rootcerts/vendor/github.com/hashicorp/go-rootcerts \
		hashicorp:go-secure-stdlib:parseutil/v0.1.7:hashicorp_go_secure_stdlib_parseutil \
		hashicorp:go-secure-stdlib:strutil/v0.1.2:hashicorp_go_secure_stdlib_strutil/vendor/github.com/hashicorp/go-secure-stdlib \
		hashicorp:go-sockaddr:v1.0.6:hashicorp_go_sockaddr/vendor/github.com/hashicorp/go-sockaddr \
		hashicorp:go-version:v1.6.0:hashicorp_go_version/vendor/github.com/hashicorp/go-version \
		hashicorp:golang-lru:v1.0.2:hashicorp_golang_lru/vendor/github.com/hashicorp/golang-lru \
		hashicorp:golang-lru:v2.0.7:hashicorp_golang_lru_v2/vendor/github.com/hashicorp/golang-lru/v2 \
		hashicorp:hcl:v1.0.1-vault-5:hashicorp_hcl/vendor/github.com/hashicorp/hcl \
		hashicorp:serf:v0.10.1:hashicorp_serf/vendor/github.com/hashicorp/serf \
		hashicorp:vault:v0.5.0:hashicorp_vault_approle/vendor/github.com/hashicorp/vault/api/auth/approle \
		hashicorp:vault:v0.5.0:hashicorp_vault_kubernetes/vendor/github.com/hashicorp/vault/api/auth/kubernetes \
		hashicorp:vault:v0.5.0:hashicorp_vault_userpass/vendor/github.com/hashicorp/vault/api/auth/userpass \
		hashicorp:vault:v1.10.0:hashicorp_vault_api/vendor/github.com/hashicorp/vault \
		inconshreveable:mousetrap:v1.1.0:inconshreveable_mousetrap/vendor/github.com/inconshreveable/mousetrap \
		jessevdk:go-flags:v1.5.0:jessevdk_go_flags/vendor/github.com/jessevdk/go-flags \
		jmespath:go-jmespath:v0.4.0:jmespath_go_jmespath/vendor/github.com/jmespath/go-jmespath \
		josharian:intern:v1.0.0:josharian_intern/vendor/github.com/josharian/intern \
		jpillora:backoff:v1.0.0:jpillora_backoff/vendor/github.com/jpillora/backoff \
		json-iterator:go:v1.1.12:json_iterator_go/vendor/github.com/json-iterator/go \
		julienschmidt:httprouter:v1.3.0:julienschmidt_httprouter/vendor/github.com/julienschmidt/httprouter \
		klauspost:compress:v1.17.7:klauspost_compress/vendor/github.com/klauspost/compress \
		klauspost:cpuid:v2.2.6:klauspost_cpuid_v2/vendor/github.com/klauspost/cpuid/v2 \
		kubernetes-sigs:kustomize:kyaml/v0.16.0:kubernetes_sigs_kustomize/vendor/sigs.k8s.io/kustomize/kyaml \
		kubernetes-sigs:yaml:v1.4.0:kubernetes_sigs_yaml/vendor/sigs.k8s.io/yaml \
		kubernetes:apimachinery:v0.28.6:kubernetes_apimachinery/vendor/k8s.io/apimachinery \
		kubernetes:client-go:v0.28.6:kubernetes_client_go/vendor/k8s.io/client-go \
		kubernetes:klog:v2.120.1:kubernetes_klog/vendor/k8s.io/klog/v2 \
		kubernetes:kube-openapi:2dd684a91f00:kubernetes_kube_openapi/vendor/k8s.io/kube-openapi \
		kubernetes:utils:3b25d923346b:kubernetes_utils/vendor/k8s.io/utils \
		kylelemons:godebug:v1.1.0:kylelemons_godebug/vendor/github.com/kylelemons/godebug \
		mailru:easyjson:v0.7.7:mailru_easyjson/vendor/github.com/mailru/easyjson \
		mattn:go-colorable:v0.1.13:mattn_go_colorable/vendor/github.com/mattn/go-colorable \
		mattn:go-isatty:v0.0.19:mattn_go_isatty/vendor/github.com/mattn/go-isatty \
		matttproud:golang_protobuf_extensions:v1.0.4:matttproud_golang_protobuf_extensions/vendor/github.com/matttproud/golang_protobuf_extensions \
		miekg:dns:v1.1.58:miekg_dns/vendor/github.com/miekg/dns \
		minio:md5-simd:v1.1.2:minio_md5_simd/vendor/github.com/minio/md5-simd \
		minio:minio-go:v7.0.67:minio_minio_go_v7/vendor/github.com/minio/minio-go/v7 \
		minio:sha256-simd:v1.0.1:minio_sha256_simd/vendor/github.com/minio/sha256-simd \
		mitchellh:colorstring:d06e56a500db:mitchellh_colorstring/vendor/github.com/mitchellh/colorstring \
		mitchellh:go-homedir:v1.1.0:mitchellh_go_homedir/vendor/github.com/mitchellh/go-homedir \
		mitchellh:go-wordwrap:v1.0.1:mitchellh_go_wordwrap/vendor/github.com/mitchellh/go-wordwrap \
		mitchellh:mapstructure:v1.5.0:mitchellh_mapstructure/vendor/github.com/mitchellh/mapstructure \
		modern-go:concurrent:bacd9c7ef1dd:modern_go_concurrent/vendor/github.com/modern-go/concurrent \
		modern-go:reflect2:v1.0.2:modern_go_reflect2/vendor/github.com/modern-go/reflect2 \
		mongodb:mongo-go-driver:v1.13.1:mongodb_mongo_go_driver/vendor/go.mongodb.org/mongo-driver \
		monochromegane:go-gitignore:205db1a8cc00:monochromegane_go_gitignore/vendor/github.com/monochromegane/go-gitignore \
		mwitkow:go-conntrack:2f068394615f:mwitkow_go_conntrack/vendor/github.com/mwitkow/go-conntrack \
		ncw:swift:v1.0.53:ncw_swift/vendor/github.com/ncw/swift \
		oklog:run:v1.1.0:oklog_run/vendor/github.com/oklog/run \
		oklog:ulid:v1.3.1:oklog_ulid/vendor/github.com/oklog/ulid \
		open-telemetry:opentelemetry-collector:semconv/v0.93.0:open_telemetry_opentelemetry_collector_2/vendor/go.opentelemetry.io/collector/semconv \
		open-telemetry:opentelemetry-collector:featuregate/v1.0.1:open_telemetry_opentelemetry_collector/vendor/go.opentelemetry.io/collector/featuregate \
		open-telemetry:opentelemetry-collector:pdata/v1.0.1:open_telemetry_opentelemetry_collector_1/vendor/go.opentelemetry.io/collector/pdata \
		open-telemetry:opentelemetry-go-contrib:instrumentation/google.golang.org/grpc/otelgrpc/v0.47.0:open_telemetry_opentelemetry_go_contrib/vendor/go.opentelemetry.io/contrib/instrumentation/google.golang.org/grpc/otelgrpc \
		open-telemetry:opentelemetry-go-contrib:instrumentation/net/http/otelhttp/v0.47.0:open_telemetry_opentelemetry_go_contrib_1/vendor/go.opentelemetry.io/contrib/instrumentation/net/http/otelhttp \
		open-telemetry:opentelemetry-go:v1.22.0:open_telemetry_opentelemetry_go/vendor/go.opentelemetry.io/otel \
		opentracing:opentracing-go:10b1cf09e00b:opentracing_opentracing_go/vendor/github.com/opentracing/opentracing-go \
		pierrec:lz4:v4.1.19:pierrec_lz4_v4/vendor/github.com/pierrec/lz4/v4 \
		pkg:browser:5ac0b6a4141c:pkg_browser/vendor/github.com/pkg/browser \
		pkg:errors:v0.9.1:pkg_errors/vendor/github.com/pkg/errors \
		pmezard:go-difflib:5d4384ee4fb2:pmezard_go_difflib/vendor/github.com/pmezard/go-difflib \
		prometheus:alertmanager:80b3cb072fbd:prometheus_alertmanager/vendor/github.com/prometheus/alertmanager \
		prometheus:client_golang:v1.18.0:prometheus_client_golang/vendor/github.com/prometheus/client_golang \
		prometheus:client_model:v0.5.0:prometheus_client_model/vendor/github.com/prometheus/client_model \
		prometheus:common:v0.1.0:prometheus_common_sigv4/vendor/github.com/prometheus/common/sigv4 \
		prometheus:common:v0.47.0:prometheus_common/vendor/github.com/prometheus/common \
		prometheus:exporter-toolkit:v0.11.0:prometheus_exporter_toolkit/vendor/github.com/prometheus/exporter-toolkit \
		prometheus:procfs:v0.12.0:prometheus_procfs/vendor/github.com/prometheus/procfs \
		protocolbuffers:protobuf-go:v1.33.0:protocolbuffers_protobuf_go/vendor/google.golang.org/protobuf \
		rainycape:unidecode:cb7f23ec59be:rainycape_unidecode/vendor/github.com/rainycape/unidecode \
		rs:cors:v1.10.1:rs_cors/vendor/github.com/rs/cors \
		rs:xid:v1.5.0:rs_xid/vendor/github.com/rs/xid \
		ryanuber:go-glob:v1.0.0:ryanuber_go_glob/vendor/github.com/ryanuber/go-glob \
		sean-:seed:e2103e2c3529:sean__seed/vendor/github.com/sean-/seed \
		segmentio:fasthash:v1.0.3:segmentio_fasthash/vendor/github.com/segmentio/fasthash \
		sercand:kuberesolver:v5.1.1:sercand_kuberesolver_v5/vendor/github.com/sercand/kuberesolver/v5 \
		shurcooL:httpfs:f1e31cf0ba5c:shurcool_httpfs/vendor/github.com/shurcooL/httpfs \
		shurcooL:vfsgen:0d455de96546:shurcool_vfsgen/vendor/github.com/shurcooL/vfsgen \
		sirupsen:logrus:v1.9.3:sirupsen_logrus/vendor/github.com/sirupsen/logrus \
		spf13:afero:v1.11.0:spf13_afero/vendor/github.com/spf13/afero \
		spf13:pflag:v1.0.5:spf13_pflag/vendor/github.com/spf13/pflag \
		stretchr:objx:v0.5.0:stretchr_objx/vendor/github.com/stretchr/objx \
		stretchr:testify:v1.8.4:stretchr_testify/vendor/github.com/stretchr/testify \
		thanos-io:objstore:bdadaefbfe03:thanos_io_objstore/vendor/github.com/thanos-io/objstore \
		twmb:franz-go:c09dc92d2db1:twmb_franz_go_kfake \
		twmb:franz-go:v1.1.0:twmb_franz_go_kprom/vendor/github.com/twmb/franz-go \
		twmb:franz-go:v1.10.0:twmb_franz_go_kadm \
		twmb:franz-go:v1.15.4:twmb_franz_go \
		uber-go:atomic:v1.11.0:uber_go_atomic/vendor/go.uber.org/atomic \
		uber-go:goleak:v1.3.0:uber_go_goleak/vendor/go.uber.org/goleak \
		uber-go:multierr:v1.11.0:uber_go_multierr/vendor/go.uber.org/multierr \
		uber-go:zap:v1.21.0:uber_go_zap/vendor/go.uber.org/zap \
		uber:jaeger-client-go:v2.30.0:uber_jaeger_client_go/vendor/github.com/uber/jaeger-client-go \
		uber:jaeger-lib:v2.4.1:uber_jaeger_lib/vendor/github.com/uber/jaeger-lib \
		xhit:go-str2duration:v2.1.0:xhit_go_str2duration_v2/vendor/github.com/xhit/go-str2duration/v2 \
		xlab:treeprint:v1.2.0:xlab_treeprint/vendor/github.com/xlab/treeprint

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
