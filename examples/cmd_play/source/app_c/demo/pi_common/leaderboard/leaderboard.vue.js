_$pi.define("app_c/demo/pi_common/leaderboard/leaderboard.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./leaderboard.vue.tpl", "app_a/util/setup", "app_c/demo/pi_common/server/pi_common.topic", "pi_common/ui/main_root", "pi_utils/util/logger"], function (require, exports, module, direct_1, leaderboard_vue_tpl_1, setup_1, pi_common_topic_1, main_root_1, logger_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    class LeaderBoard {
        constructor() {
            this.list = [];
            this.myTop = '';
            // 生成测试数据
            this.initTest = () => {
                setup_1.clientRpc(pi_common_topic_1.leaderborardInit).then(r => {
                    logD('initTest!!!r:', r);
                });
            };
            // 获取排行榜
            this.getTop = () => {
                setup_1.clientRpc(pi_common_topic_1.leaderborardTop).then(r => {
                    logD('getTop!!!r:', r);
                    this.list = r;
                });
            };
            // 获取我的排名
            this.myRank = () => {
                setup_1.clientRpc(pi_common_topic_1.getMyRank).then(r => {
                    logD('getMyRank!!!r:', r);
                    this.myTop = r;
                });
            };
            this.closePage = () => {
                main_root_1.close(this);
            };
        }
    }
    exports.default = LeaderBoard;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/pi_common/leaderboard/leaderboard.vue.tpl.ts",
            _$cssPath = "app_c/demo/pi_common/leaderboard/leaderboard.vue.wcss",
            _$cssHash = 2243081170;
        LeaderBoard["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: leaderboard_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(LeaderBoard, ["list", "myTop"]);
    direct_1.addField(LeaderBoard, ['initTest', 'getTop', 'myRank', 'closePage']);
});