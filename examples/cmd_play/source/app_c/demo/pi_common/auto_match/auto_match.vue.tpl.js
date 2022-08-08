_$pi.define("app_c/demo/pi_common/auto_match/auto_match.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue", "app_a/widget/btn/btn.vue"], function (require, exports, module, direct_1, imgmap_vue_1, btn_vue_1) {
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
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "创建房间"
                },
                style: {
                    width /*width*/: "180px",
                    height /*height*/: "70px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.createRoom($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n4 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "自动匹配"
                },
                style: {
                    width /*width*/: "180px",
                    height /*height*/: "70px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.autoMatch($event);
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n5 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "dialog_close_btn"
                },
                class: 452908938,
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
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.mountChildWidget(t.n4, t.n2);
            direct_1.mountChildWidget(t.n5, t.n2);
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