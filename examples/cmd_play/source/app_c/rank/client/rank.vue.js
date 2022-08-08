_$pi.define("app_c/rank/client/rank.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./rank.vue.tpl", "pi_utils/util/logger", "app_b/main_container/main_container.vue", "app_c/demo/pi_common/server/pi_common.topic", "app_a/util/setup", "pi_common/store/store", "app_a/struct/db_role.struct"], function (require, exports, module, direct_1, rank_vue_tpl_1, logger_1, main_container_vue_1, pi_common_topic_1, setup_1, store_1, db_role_struct_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    class Rank {
        constructor() {
            this.rankList = [];
            this.selfRank = 0;
            this.arr = [{ title: '排名', width: 144 }, { title: '玩家', width: 300 }, { title: '爱心值', width: 180 }];
            this.role = { rid: '', heart: 0 };
        }
        create() {
            this.init();
            main_container_vue_1.WIDGET_MAP.set(Rank, { _widget: this, _sceneName: '' });
        }
        // 当进入后直接切到其他界面会找不到位置
        attach() {
            // 配合底部菜单使用
            main_container_vue_1.SCREEN_HANDLER.notify(main_container_vue_1.ScreenEvent.WidgetPainted, [Rank]);
        }
        update() {
            setup_1.clientRpc(pi_common_topic_1.leaderborardInit).then(r => {
                logD('initTest!!!r:', r);
                this.init();
            });
        }
        init() {
            setup_1.clientRpc(pi_common_topic_1.leaderborardTop).then(r => {
                logD('getTop!!!r:', r);
                this.rankList = r;
            });
            this.role = store_1.find(db_role_struct_1.CurrencyDb);
            setup_1.clientRpc(pi_common_topic_1.getMyRank).then(r => {
                logD('getMyRank!!!r:', r);
                this.selfRank = r;
            });
        }
    }
    exports.default = Rank;
    exports.initMeta = () => {
        let _$tpl = "app_c/rank/client/rank.vue.tpl.ts",
            _$cssPath = "app_c/rank/client/rank.vue.wcss",
            _$cssHash = 1797309432;
        Rank["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: rank_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Rank, ["rankList", "arr", "selfRank", "role"]);
});