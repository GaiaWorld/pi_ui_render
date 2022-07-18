_$pi.define("app_c/demo/pi_common/auto_match/auto_match.vue", ["require", "exports", "module", "./auto_match.vue.tpl", "app_a/util/setup", "app_c/demo/pi_common/server/pi_common.topic", "pi_common/ui/main_root", "pi_utils/util/logger"], function (require, exports, module, auto_match_vue_tpl_1, setup_1, pi_common_topic_1, main_root_1, logger_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    class AutoMatch {
        createRoom() {
            setup_1.clientRpc(pi_common_topic_1.createRoom).then(r => {
                logD('createRoom!!!r:', r);
            });
        }
        autoMatch() {
            setup_1.clientRpc(pi_common_topic_1.autoMatch).then(r => {
                logD('autoMatch!!!r:', r);
            });
        }
        closePage() {
            main_root_1.close(this);
        }
    }
    exports.default = AutoMatch;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/pi_common/auto_match/auto_match.vue.tpl.ts",
            _$cssPath = "app_c/demo/pi_common/auto_match/auto_match.vue.wcss",
            _$cssHash = 3310795104;
        AutoMatch["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: auto_match_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
});