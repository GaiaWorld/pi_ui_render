_$pi.define("app_c/demo/ui/show/base/canvas.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0, 300], [0, 400], [0, 0, 0, 1], [0, 40], [0, 0, 1, 1], [0, 1, 0, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        s3() {
            let w = this.w;
            return !w.isDeleteCanvas1 ? B3 : null;
        }

        s8() {
            let w = this.w;
            return !w.isDeleteCanvas2 ? B8 : null;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.i3 = t.s3();
            t.n3 = direct_1.createIf(w, t.i3);
            t.i8 = t.s8();
            t.n8 = direct_1.createIf(w, t.i8);
            return this.n2;
        }
        d() {
            let t = this;
            direct_1.destroyContext(t.n3);
            direct_1.destroyContext(t.n8);
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            t.n3.m(t.n2);
            t.n8.m(t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            t.n3 = direct_1.patchIf(w, t.n3, t.i3, t.i3 = t.s3(), t.n2);
            t.n8 = direct_1.patchIf(w, t.n8, t.i8, t.i8 = t.s8(), t.n2);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
    class B8 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n8 = direct_1.createDiv();
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n8, 22 /*flexWrap*/, 1);
            t.n9 = direct_1.createCanvas();
            direct_1.setAttr(t.n9, "name", "canvas2");
            direct_1.setAttr(t.n9, "width", "300");
            direct_1.setAttr(t.n9, "height", "400");
            direct_1.setAttr(t.n9, "ref", "canvas2");
            direct_1.setStyle(t.n9, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n9, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n9, 46 /*backgroundColor*/, staticObj[3]);
            t.n10 = direct_1.createDiv();
            t.n11 = direct_1.createSpan();
            direct_1.setStyle(t.n11, 1 /*height*/, staticObj[4]);
            direct_1.setStyle(t.n11, 46 /*backgroundColor*/, staticObj[5]);
            direct_1.setEvent(t.n11, "pointerclick", $event => {
                let r = w.drawByGui2();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n11, "绘制canvas");
            t.n12 = direct_1.createSpan();
            direct_1.setStyle(t.n12, 1 /*height*/, staticObj[4]);
            direct_1.setStyle(t.n12, 46 /*backgroundColor*/, staticObj[6]);
            direct_1.setEvent(t.n12, "pointerclick", $event => {
                let r = w.deleteCanvas3();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n12, "删除canvas");
            return this.n8;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n9, t.n8);
            direct_1.append(t.n11, t.n10);
            direct_1.append(t.n12, t.n10);
            direct_1.append(t.n10, t.n8);
            direct_1.insertBefore(t.n8, target, anchor);
        }
        p() {
            return this.n8;
        }
    }
    class B3 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n3, 22 /*flexWrap*/, 1);
            t.n4 = direct_1.createCanvas();
            direct_1.setAttr(t.n4, "name", "canvas1");
            direct_1.setAttr(t.n4, "width", "300");
            direct_1.setAttr(t.n4, "height", "300");
            direct_1.setAttr(t.n4, "ref", "canvas1");
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n4, 46 /*backgroundColor*/, staticObj[3]);
            t.n5 = direct_1.createDiv();
            t.n6 = direct_1.createSpan();
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[4]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[5]);
            direct_1.setEvent(t.n6, "pointerclick", $event => {
                let r = w.drawByGui1();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n6, "绘制canvas");
            t.n7 = direct_1.createSpan();
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[4]);
            direct_1.setStyle(t.n7, 46 /*backgroundColor*/, staticObj[6]);
            direct_1.setEvent(t.n7, "pointerclick", $event => {
                let r = w.deleteCanvas1();
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n7, "删除canvas");
            return this.n3;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n7, t.n5);
            direct_1.append(t.n5, t.n3);
            direct_1.insertBefore(t.n3, target, anchor);
        }
        p() {
            return this.n3;
        }
    }
});