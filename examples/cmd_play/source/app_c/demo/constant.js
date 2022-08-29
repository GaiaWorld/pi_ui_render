_$pi.define("app_c/demo/constant", ["require", "exports", "module", "pi_common/ui/main_root", "pi_utils/util/logger", "app_a/util/setup", "app_c/demo/pi_common/server/pi_common.topic", "app_c/demo/pi_common/pay/pay.vue", "app_c/demo/pi_common/leaderboard/leaderboard.vue", "app_b/feat/page/test.vue", "pi_common/sdk/index", "pi_sys/index", "pi_common/sso/sso_client", "pi_common/sdk/interface/index", "pi_common/id_verify/id_verify", "pi_common/id_verify/verify.vue", "app_c/chat/world_chat.vue", "app_c/demo/pi_common/tween/tween.vue", "app_c/demo/pi_common/web3/web3.vue", "app_c/fight/fight_main.vue"], function (require, exports, module, main_root_1, logger_1, setup_1, pi_common_topic_1, pay_vue_1, leaderboard_vue_1, test_vue_1, index_1, index_2, sso_client_1, index_3, id_verify_1, verify_vue_1, world_chat_vue_1, tween_vue_1, web3_vue_1, fight_main_vue_1) {
    "use strict";

    exports.ServerCfgList = exports.PiCommonCfgList = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    let sdk;
    // pi_common的功能
    exports.PiCommonCfgList = [['SDK初始化', function () {
        try {
            return Promise.resolve(index_1.getSdkContext()).then(function (_index_1$getSdkContex) {
                sdk = _index_1$getSdkContex;
                // 总是要配置 单点登录服务器；
                let { host, port, isTls } = index_2.PISYS.Env.get('platform_server').default;
                sdk.login.setServer(host, port, isTls);
                sdk.login.saveAuth(true);
                sdk.login.useLastAuth(true);
                sdk.login.onLogin((info, fail) => {
                    if (fail) {
                        logI("登录失败, fail = ", fail);
                        // 弹提示 || 重新登录一次
                    } else {
                        logI("登录成功：sdkName = " + info.sdkName);
                        // 开始授权
                        sdk.login.getAuth().then(auth => {
                            logI("授权成功，auth = ", auth);
                        }).catch(e => {
                            logE("授权失败，reason = ", e);
                        });
                        // 登录成功之后，上报用户信息
                        // 可以不在这里调用，但必须要在登录成功之后，支付之前，仅调用一次；
                    }
                });
                sdk.login.onLogout((info, fail) => {
                    if (fail) {
                        logI("登出失败, fail = ", fail);
                    } else {
                        logI("登出成功：sdkName = " + info.sdkName);
                        // 清除凭证，具体说明看 注释
                        sdk.login.clearAuth();
                    }
                });
            });
        } catch (e) {
            return Promise.reject(e);
        }
    }], ['登录', () => {
        // 授权先行，因为会有可能使用上次登录凭证
        sdk.login.getAuth(index_2.PISYS.Env.get("name")).then(auth => {
            logI("授权成功，auth = ", auth);
            // 成功，拿auth和游戏服务器通信吧！！！
        }).catch(e => {
            logI("授权失败，reason = ", e);
            // 授权失败才会调用登录，没有返回值；成功与否通过 onLogin 判断
            sdk.login.login({
                loginType: sso_client_1.LoginType.ACCOUNT,
                autoRegister: true,
                account: 'aaa',
                pwd: '123456'
            });
        });
    }], ['登出', () => {
        // 没有返回值；成功与否通过 onLogout 判断
        sdk.login.logout();
    }], ['修改密码', () => {
        sdk.login.changePwd('aaa', 'aaa', '888888').then(r => {
            console.log('修改成功', r);
        }).catch(err => {
            console.log('修改失败', err);
        });
    }], ['分享', function () {
        try {
            return Promise.resolve(new Promise(resolve => {
                sdk.share.shareText({
                    shareType: index_3.ShareType.qq,
                    text: '哈咯'
                }).then(info => {
                    resolve('分享成功 info: ' + JSON.stringify(info));
                }).catch(e => {
                    resolve('分享失败 e: ' + JSON.stringify(e));
                });
            }));
        } catch (e) {
            return Promise.reject(e);
        }
    }], ['上报事件', () => {
        const userInfo = {
            serverID: "server_1",
            userID: '10001',
            uuid: 'abcde'
        };
        sdk.upload.login(userInfo);
    }], ['防沉迷', () => {
        id_verify_1.identityVerification.idVerifySuccess(r => {
            console.log('id idVerify success', r);
        });
        id_verify_1.identityVerification.idVerifyError(r => {
            console.log('id idVerify error', r);
        });
        let { host, port, isTls } = index_2.PISYS.Env.get('platform_server').default;
        id_verify_1.identityVerification.query(host, port, isTls, 'ydzm', 'abcde', setup_1.clientRpc, true).then(r => {
            // 未实名
            if (!r) {
                main_root_1.open(verify_vue_1.default);
            }
        });
    }], ['支付', () => {
        main_root_1.pop(pay_vue_1.default);
    }],
    // [  // 待重构
    //     '自动匹配', () => {
    //         pop(AutoMatch)
    //     }
    // ],
    ['排行榜', () => {
        main_root_1.pop(leaderboard_vue_1.default);
    }], ['社交', () => {
        main_root_1.open(world_chat_vue_1.default);
    }], ['测试数据监听', () => {
        // 调用后端接口推送数据
        setup_1.clientRpc(pi_common_topic_1.testDbMonitor).then(r => {
            logD('testDbMonitor:', r);
        });
    }], ['消息推送(短信)', () => {
        setup_1.clientRpc(pi_common_topic_1.templatePhone, '15308216675').then(r => {
            logD('templatePhone!!!r:', r);
        });
    }], ['消息推送(公众号)', () => {
        setup_1.clientRpc(pi_common_topic_1.templatePub).then(r => {
            logD('templatePub!!!r:', r);
        });
    }], ['消息推送(APP)', () => {
        setup_1.clientRpc(pi_common_topic_1.templateApp).then(r => {
            logD('templateApp!!!r:', r);
        });
    }], ['tween', () => {
        main_root_1.open(tween_vue_1.default);
    }], ['fight', () => {
        main_root_1.open(fight_main_vue_1.default);
    }], ['测试', () => {
        main_root_1.open(test_vue_1.default);
    }], ['web3测试', () => {
        main_root_1.open(web3_vue_1.default);
    }]];
    // 服务器相关的功能
    exports.ServerCfgList = [];
});