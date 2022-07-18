_$pi.define("app_b/feat/page/test.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue", "app_a/widget/count_down/count_down.vue", "app_a/widget/increaser/increaser.vue"], function (require, exports, module, direct_1, btn_vue_1, count_down_vue_1, increaser_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1, 1, 1], [1, 1]];
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
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "返回",
                    fontSize: "26"
                },
                style: {
                    width /*width*/: "180px",
                    height /*height*/: "90px",
                    margin /*margin*/: "5px 10px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.closePage($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n4 = direct_1.createWidget(increaser_vue_1.default, w, {
                attrs: {
                    number: w.number
                },
                style: {
                    width /*width*/: "100%",
                    height /*height*/: "50px"
                }
            });
            t.n5 = direct_1.createWidget(count_down_vue_1.default, w, {
                attrs: {
                    nextTime: w.nextTime
                },
                style: {
                    width /*width*/: "100%",
                    height /*height*/: "50px"
                },
                events: {
                    "ev-timeEnd": $event => {
                        let r = w.timeEnd($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n6 = direct_1.createDiv();
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n6, 22 /*flexWrap*/, 1);
            t.n7 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    text: "增加"
                },
                style: {
                    width /*width*/: "auto",
                    height /*height*/: "80px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.increase($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n8 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    text: "倒计时"
                },
                style: {
                    width /*width*/: "auto",
                    height /*height*/: "80px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.setTimer($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n9 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    text: "弹窗"
                },
                style: {
                    width /*width*/: "auto",
                    height /*height*/: "80px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.popUp($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n10 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    fontSize: "26",
                    text: "宝箱动画",
                    metaImg: "btn_meta_play"
                },
                style: {
                    width /*width*/: "auto",
                    height /*height*/: "80px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "ev-click": $event => {
                        let r = w.openTreasure($event);
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
            direct_1.destroyContext(t.n7);
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n9);
            direct_1.destroyContext(t.n10);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.mountChildWidget(t.n4, t.n2);
            direct_1.mountChildWidget(t.n5, t.n2);
            direct_1.mountChildWidget(t.n7, t.n6);
            direct_1.mountChildWidget(t.n8, t.n6);
            direct_1.mountChildWidget(t.n9, t.n6);
            direct_1.mountChildWidget(t.n10, t.n6);
            direct_1.append(t.n6, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            var _$wAttrs = {};
            if (dirty0 & 1) {
                const _$attrs = {};
                if (dirty0 & 1) _$attrs.number = w.number;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n4, _$wAttrs);
            var _$wAttrs = {};
            if (dirty0 & 2) {
                const _$attrs = {};
                if (dirty0 & 2) _$attrs.nextTime = w.nextTime;
                _$wAttrs.attrs = _$attrs;
            }
            ;
            direct_1.patchAttrsForWidget(t.n5, _$wAttrs);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});