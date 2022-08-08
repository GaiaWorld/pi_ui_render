_$pi.define("app_c/demo/ui/show/component/psdimg.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct", "pi_gui/ui/imgmap.vue"], function (require, exports, module, direct_1, imgmap_vue_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 700]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "tubiao__602506"
                },
                style: {
                    position /*position*/: "relative"
                }
            });
            t.n4 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "tubiao__602506"
                },
                style: {
                    position /*position*/: "relative",
                    width /*width*/: "200px",
                    height /*height*/: "200px",
                    marginLeft /*marginLeft*/: "10px",
                    backgroundColor /*backgroundColor*/: "rgb(255,0,0)"
                }
            });
            t.n5 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "win_bg"
                },
                style: {
                    position /*position*/: "relative"
                }
            });
            t.n6 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "win_bg"
                },
                style: {
                    position /*position*/: "relative",
                    width /*width*/: "400px",
                    height /*height*/: "200px",
                    marginLeft /*marginLeft*/: "20px",
                    marginTop /*marginTop*/: "20px"
                }
            });
            t.n7 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "box_orange"
                },
                style: {
                    position /*position*/: "relative",
                    backgroundColor /*backgroundColor*/: "rgb(255,0,0)",
                    margin /*margin*/: "10px"
                }
            });
            t.n8 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "box_orange"
                },
                class: 2448521264
            });
            t.n9 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "card_shangjiazhichi"
                },
                style: {
                    position /*position*/: "relative"
                }
            });
            t.n10 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "card_shangjiazhichi",
                    imageClip: [0.2, 0.2, 0.2, 0.2]
                },
                style: {
                    position /*position*/: "relative"
                }
            });
            t.n11 = direct_1.createWidget(imgmap_vue_1.default, w, {
                attrs: {
                    name: "card_shangjiazhichi_head"
                },
                style: {
                    position /*position*/: "relative"
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