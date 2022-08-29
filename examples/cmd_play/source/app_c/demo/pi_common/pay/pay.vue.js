_$pi.define("app_c/demo/pi_common/pay/pay.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./pay.vue.tpl", "pi_common/ui/main_root", "pi_sys/index", "pi_utils/util/logger", "app_a/util/setup", "pi_common/sdk/interface/pay", "app_c/demo/pi_common/server/pi_common.topic"], function (require, exports, module, direct_1, pay_vue_tpl_1, main_root_1, index_1, logger_1, setup_1, pay_1, pi_common_topic_1) {
    "use strict";

    exports.initMeta = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, "app");
    const func = r => {
        setup_1.Global.sdk.pay.queryOrder("test", r.pt_oid).then(order => {
            logD("查询订单成功", order);
        }).catch(err => {
            logD("查询订单失败", err);
        });
    };
    class Pay {
        constructor() {
            this.msg = "点我支付";
        }

        click(type) {
            try {
                let channel = pay_1.PayChannelName[type];
                let { host, port, isTls } = index_1.PISYS.Env.get("platform_server").default;
                setup_1.Global.sdk.pay.setServer(host, port, isTls);
                setup_1.Global.sdk.pay.unifiedOrder({
                    app: "test",
                    channel,
                    amount: 1,
                    currency: "cny"
                }).then(r => {
                    logD("下单成功", r);
                    const info = JSON.parse(r.credential).info;
                    let payInfo;
                    payInfo = {
                        amount: 1,
                        orderID: r.pt_oid,
                        goodsID: "2",
                        goodsDesc: "test",
                        goodsName: "测试",
                        orderInfo: info,
                        payment: pay_1.PayType[type]
                    };
                    setup_1.Global.sdk.pay.pay(payInfo).then(() => {
                        func(r);
                    }).catch(e => {
                        logE("支付失败", e);
                    });
                }).catch(err => {
                    logE("下单失败", err);
                });
                return Promise.resolve();
            } catch (e) {
                return Promise.reject(e);
            }
        }

        supplement() {
            setup_1.Global.sdk.pay.supplementOrders().then(r => {
                logD("补单结束，结果 r = ", r);
            });
        }
        search() {
            setup_1.clientRpc(pi_common_topic_1.reconciliationDtimer).then(r => {
                logD("reconciliationDtimer!!!r:", r);
            });
        }
        closePage() {
            main_root_1.close(this);
        }
    }
    exports.default = Pay;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/pi_common/pay/pay.vue.tpl.ts",
            _$cssPath = "app_c/demo/pi_common/pay/pay.vue.wcss",
            _$cssHash = 3738085676;
        Pay["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: pay_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.addField(Pay, ['msg']);
});