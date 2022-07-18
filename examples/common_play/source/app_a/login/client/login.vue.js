_$pi.define("app_a/login/client/login.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./login.vue.tpl", "pi_gui/widget/direct", "pi_common/ui/tools", "pi_common/sso/sso_client", "app_a/util/setup", "pi_utils/res/sound", "pi_common/ui/main_root", "pi_utils/util/logger", "./register.vue", "app_a/widget/tips/tips.vue"], function (require, exports, module, direct_1, login_vue_tpl_1, direct_2, tools_1, sso_client_1, setup_1, sound_1, main_root_1, logger_1, register_vue_1, tips_vue_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, 'app');
    class Login {
        constructor() {
            this.loginType = sso_client_1.LoginType.PHONE;
            this.input1 = '';
            this.input2 = '';
            this.down = id => {
                tools_1.playScaleAnimation(direct_2.findElementByAttr(this, 'id', id), 1, 0.8);
            };
            this.up = id => {
                tools_1.playScaleAnimation(direct_2.findElementByAttr(this, 'id', id), 0.8, 1);
            };
        }
        // 支付测试 直接调用底层支付sdk
        payTest() {
            let payInfo = {
                amount: 1,
                orderID: '123456',
                goodsName: '测试',
                orderInfo: '',
                goodsID: '124'
            };
            setup_1.Global.sdk.pay.pay(payInfo).then(r => {
                logI('支付成功', r);
            }).catch(e => {
                logE('支付失败', e);
            });
        }
        doLogin(loginType) {
            switch (loginType) {
                case sso_client_1.LoginType.ACCOUNT:
                    setup_1.Global.sdk.login.login({
                        loginType: sso_client_1.LoginType.ACCOUNT,
                        account: this.input1,
                        pwd: this.input2
                    });
                    break;
                case sso_client_1.LoginType.PHONE:
                    setup_1.Global.sdk.login.login({
                        loginType: sso_client_1.LoginType.PHONE,
                        phone: this.input1,
                        code: this.input2
                    });
                    break;
                case sso_client_1.LoginType.VISITORS:
                    setup_1.Global.sdk.login.login({
                        loginType: sso_client_1.LoginType.VISITORS
                    });
                    break;
                case sso_client_1.LoginType.QQ:
                    setup_1.Global.sdk.login.login({
                        loginType: sso_client_1.LoginType.QQ
                    });
                    break;
                case sso_client_1.LoginType.WX:
                    setup_1.Global.sdk.login.login({
                        loginType: sso_client_1.LoginType.WX
                    });
                    break;
                case sso_client_1.LoginType.WB:
                    setup_1.Global.sdk.login.login({
                        loginType: sso_client_1.LoginType.WB
                    });
                    break;
                default:
                    throw new Error('暂未支持');
            }
        }
        handleLogin() {
            sound_1.BGMusic.server = 's';
            sound_1.BGMusic.play('app_a/music/bg_2.mp3');
            if (!this.input1 || !this.input2) {
                const message = this.loginType === sso_client_1.LoginType.ACCOUNT ? '请填写账号密码' : '请填写手机号和验证码';
                tips_vue_1.showTips(message);
                return Promise.resolve();
            }
            return this.doLogin(this.loginType);
        }
        // 游戏自己登录逻辑
        input1Change(e) {
            this.input1 = e.current._value;
        }
        input2Change(e) {
            this.input2 = e.current._value;
        }
        // 切换登录类型
        switchLoginType(lType) {
            if (this.loginType === lType) return;
            this.input1 = '';
            this.input2 = '';
            this.loginType = lType;
        }
        // 获取验证码
        getVerifyCode() {
            if (this.countDown) return;
            setup_1.Global.sdk.login.getCode(this.input1).then(code => {
                logD('!!!!!!!!!!!!!!!!!! code ', code);
                this.countDown = 60;
                const timer = setInterval(() => {
                    --this.countDown;
                    if (!this.countDown) {
                        clearInterval(timer);
                    }
                }, 1000);
            }).catch(e => {
                logD('getVerifyCode failed, error = ', e);
                tips_vue_1.showTips(e.message);
            });
        }
        touristLoginClick() {
            this.doLogin(sso_client_1.LoginType.VISITORS);
        }
        registerAccount() {
            main_root_1.pop(register_vue_1.default, null, null, {
                cancel: () => {
                    logD('cancel!!!!!!!!!!!!!!!!!!!!!!!');
                    main_root_1.close(this);
                }
            });
        }
    }
    exports.default = Login;
    exports.initMeta = () => {
        let _$tpl = "app_a/login/client/login.vue.tpl.ts",
            _$cssPath = "app_a/login/client/login.vue.wcss",
            _$cssHash = 1437978939;
        Login["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: login_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Login, ["loginType", "input1", "countDown", "input2"]);
    direct_1.addField(Login, ['down', 'up']);
});