_$pi.define("app_c/demo/pi_common/pay/pay.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, btn_vue_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1, 1, 1], [1, 1], [[0, 10], null, null, null, null]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 46 /*backgroundColor*/, staticObj[0]);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "Quick支付"
                },
                class: 1628997160,
                events: {
                    "pointerclick": $event => {
                        let r = w.click('quicksdk');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n4 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "ohy_sdk支付"
                },
                class: 1628997160,
                events: {
                    "pointerclick": $event => {
                        let r = w.click('ohy_sdk');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n5 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "微信支付"
                },
                class: 1628997160,
                events: {
                    "pointerclick": $event => {
                        let r = w.click('wx');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n6 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "支付宝支付"
                },
                class: 1628997160,
                events: {
                    "pointerclick": $event => {
                        let r = w.click('zfb');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n7 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "微信网页支付 "
                },
                class: 1628997160,
                events: {
                    "pointerclick": $event => {
                        let r = w.click('wx_web');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n8 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "支付宝网页支付"
                },
                class: 1628997160,
                events: {
                    "pointerclick": $event => {
                        let r = w.click('zfb_web');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n9 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "steam支付"
                },
                class: 1628997160,
                events: {
                    "pointerclick": $event => {
                        let r = w.click('steam');
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n10 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "补单"
                },
                class: 1628997160,
                events: {
                    "pointerclick": $event => {
                        let r = w.supplement($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n11 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "对账查询"
                },
                class: 1628997160,
                events: {
                    "pointerclick": $event => {
                        let r = w.search($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n12 = direct_1.createDiv();
            direct_1.setStyle(t.n12, 11 /*padding*/, staticObj[2]);
            t.n13 = direct_1.createSpan();
            direct_1.setClass(t.n13, 3769313102);
            direct_1.setText(t.n13, "注：渠道包会默认使用渠道SDK支付");
            t.n14 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "dialog_close_btn"
                },
                class: 3290937859,
                events: {
                    "pointerclick": $event => {
                        let r = w.closePage($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n4);
            direct_1.destroyContext(t.n5);
            direct_1.destroyContext(t.n6);
            direct_1.destroyContext(t.n7);
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n9);
            direct_1.destroyContext(t.n10);
            direct_1.destroyContext(t.n11);
            direct_1.destroyContext(t.n14);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.mountChildWidget(t.n4, t.n2);
            direct_1.mountChildWidget(t.n5, t.n2);
            direct_1.mountChildWidget(t.n6, t.n2);
            direct_1.mountChildWidget(t.n7, t.n2);
            direct_1.mountChildWidget(t.n8, t.n2);
            direct_1.mountChildWidget(t.n9, t.n2);
            direct_1.mountChildWidget(t.n10, t.n2);
            direct_1.mountChildWidget(t.n11, t.n2);
            direct_1.append(t.n13, t.n12);
            direct_1.append(t.n12, t.n2);
            direct_1.mountChildWidget(t.n14, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let {} = w._$info;
            direct_1.extendAttr(t.n2, w);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});