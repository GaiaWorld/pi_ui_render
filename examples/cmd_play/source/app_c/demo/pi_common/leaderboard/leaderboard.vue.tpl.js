_$pi.define("app_c/demo/pi_common/leaderboard/leaderboard.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "app_a/widget/btn/btn.vue", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, btn_vue_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1, 1, 1], [1, 1], [[0, 5], null, null, null, null], [1, 0, 0, 1], [0.13, 0.13, 0.13, 1]];
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
                    text: "初始化数据"
                },
                style: {
                    width /*width*/: "220px",
                    height /*height*/: "70px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.initTest();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n4 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "获取排行"
                },
                style: {
                    width /*width*/: "180px",
                    height /*height*/: "70px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.getTop();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n5 = direct_1.createWidget(btn_vue_1.default, w, {
                attrs: {
                    text: "我的排名"
                },
                style: {
                    width /*width*/: "180px",
                    height /*height*/: "70px",
                    margin /*margin*/: "10px"
                },
                events: {
                    "pointerclick": $event => {
                        let r = w.myRank();
                        $event && typeof $event === "object" && ($event.stopPropagation = r);
                        return r;
                    }
                }
            });
            t.n6 = direct_1.createDiv();
            direct_1.setAttr(t.n6, "scroll_type", "none");
            direct_1.setAttr(t.n6, "layout", "scroll");
            direct_1.setAttr(t.n6, "scroll_path", "y");
            direct_1.setAttr(t.n6, "id", "list");
            direct_1.setClass(t.n6, 3358638484);
            t.n7 = direct_1.createDiv();
            direct_1.setClass(t.n7, 1101376341);
            t.n8 = direct_1.createFor(w, w.list, B8);
            t.n11 = direct_1.createDiv();
            direct_1.setClass(t.n11, 3051829206);
            t.n12 = direct_1.createSpan();
            direct_1.setStyle(t.n12, 30 /*color*/, staticObj[4]);
            direct_1.setStyle(t.n12, 42 /*fontSize*/, 30);
            direct_1.setText(t.n12, w.myTop);
            t.n13 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "dialog_close_btn"
                },
                class: 1385411475,
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
            direct_1.destroyContext(t.n8);
            direct_1.destroyContext(t.n13);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.mountChildWidget(t.n3, t.n2);
            direct_1.mountChildWidget(t.n4, t.n2);
            direct_1.mountChildWidget(t.n5, t.n2);
            t.n8.m(t.n7);
            direct_1.append(t.n7, t.n6);
            direct_1.append(t.n6, t.n2);
            direct_1.append(t.n12, t.n11);
            direct_1.append(t.n11, t.n2);
            direct_1.mountChildWidget(t.n13, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            direct_1.patchFor(w, t.n8, w.list, B8);
            if (dirty0 & 2) direct_1.setText(t.n12, w.myTop);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B8 {
        constructor(widget, ctx) {
            this.w = widget;
            this._$ctx = ctx || widget._$info.ctx;
        }
        c() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            t.n8 = direct_1.createDiv();
            direct_1.setAttr(t.n8, "key", v.uid);
            direct_1.setStyle(t.n8, 16 /*margin*/, staticObj[2]);
            t.n9 = direct_1.createSpan();
            direct_1.setStyle(t.n9, 30 /*color*/, staticObj[3]);
            direct_1.setStyle(t.n9, 42 /*fontSize*/, 30);
            direct_1.setText(t.n9, i + 1);
            t.n10 = direct_1.createSpan();
            direct_1.setStyle(t.n10, 30 /*color*/, staticObj[4]);
            direct_1.setStyle(t.n10, 42 /*fontSize*/, 30);
            direct_1.setText(t.n10, JSON.stringify(v));
            return this.n8;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n10, t.n8);
            direct_1.insertBefore(t.n8, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let [v, i] = t._$ctx;
            let { dirty0 } = w._$info;
            direct_1.setAttr(t.n8, "key", v.uid);
            direct_1.setText(t.n9, i + 1);
            direct_1.setText(t.n10, JSON.stringify(v));
            return this.n8;
        }
        s(v, i) {
            let w = this.w;
            this._$ctx[0] = v;
            this._$ctx[1] = i;
        }
        getKey(v, i) {
            return v.uid;
        }
    }
});