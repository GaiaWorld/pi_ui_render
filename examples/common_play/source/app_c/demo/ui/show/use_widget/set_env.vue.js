_$pi.define("app_c/demo/ui/show/use_widget/set_env.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./set_env.vue.tpl", "pi_gui/widget/direct", "pi_common/ui/main_root"], function (require, exports, module, direct_1, set_env_vue_tpl_1, direct_2, main_root_1) {
    "use strict";

    exports.initMeta = void 0;
    /// 在应用初始化时就可以设置全局环境，写在这里，只是为了测试方便
    let env = 1;
    direct_2.WidgetBase.set_env(env);
    class SetEnv extends direct_2.WidgetBase {
        change() {
            env = -env;
            direct_2.WidgetBase.set_env(env);
            main_root_1.getRoot().reset();
        }
    }
    exports.default = SetEnv;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/use_widget/set_env.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/use_widget/set_env.vue.wcss",
            _$cssHash = 3440625589;
        SetEnv["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: set_env_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(SetEnv, ["env"]);
});