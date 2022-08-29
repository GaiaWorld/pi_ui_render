_$pi.define("app_a/login/client/register.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./register.vue.tpl", "pi_common/sdk/index", "pi_common/sso/sso_client", "pi_common/ui/main_root", "app_a/widget/tips/tips.vue"], function (require, exports, module, direct_1, register_vue_tpl_1, index_1, sso_client_1, main_root_1, tips_vue_1) {
    "use strict";

    exports.initMeta = exports.closeRegister = void 0;
    let w;
    class Register {
        constructor() {
            this.account = "";
            this.pwd = "";
        }
        create() {
            w = this;
        }
        accountChange(e) {
            this.account = e.current._value;
        }
        pwdChange(e) {
            this.pwd = e.current._value;
        }

        registerClick() {
            try {
                const _this = this;

                if (!_this.account || !_this.account) {
                    tips_vue_1.showTips("请填写账号密码");
                    return Promise.resolve();
                }
                return Promise.resolve(index_1.getSdkContext()).then(function (sdk) {
                    sdk.login.login({
                        loginType: sso_client_1.LoginType.ACCOUNT,
                        account: _this.account,
                        pwd: _this.pwd,
                        autoRegister: true
                    });
                });
            } catch (e) {
                return Promise.reject(e);
            }
        }

        closeClick() {
            main_root_1.close(w);
        }
    }
    exports.default = Register;
    exports.closeRegister = () => {
        w && main_root_1.close(w);
        w = null;
    };
    exports.initMeta = () => {
        let _$tpl = "app_a/login/client/register.vue.tpl.ts",
            _$cssPath = "app_a/login/client/register.vue.wcss",
            _$cssHash = 885375710;
        Register["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: register_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Register, ["account", "pwd"]);
});