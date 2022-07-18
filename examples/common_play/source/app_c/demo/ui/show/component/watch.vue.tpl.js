_$pi.define("app_c/demo/ui/show/component/watch.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0, 30], [0, 100], [0, 50], [0, 1, 0, 1], [0, 10]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[0]);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[0]);
            t.n4 = direct_1.createSpan();
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[2]);
            direct_1.setText(t.n4, "a1:" + w.a1);
            t.n5 = direct_1.createSpan();
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n5, 0 /*width*/, staticObj[2]);
            direct_1.setText(t.n5, "a2:" + w.a2);
            t.n6 = direct_1.createSpan();
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[2]);
            direct_1.setText(t.n6, "a3:" + w.a3);
            t.n7 = direct_1.createSpan();
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n7, 0 /*width*/, staticObj[2]);
            direct_1.setText(t.n7, "a4:" + w.a4);
            t.n8 = direct_1.createSpan();
            direct_1.setStyle(t.n8, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n8, 0 /*width*/, staticObj[2]);
            direct_1.setText(t.n8, "a5:" + w.a5);
            t.n9 = direct_1.createDiv();
            direct_1.setStyle(t.n9, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n9, 22 /*flexWrap*/, 1);
            t.n10 = direct_1.createSpan();
            direct_1.setStyle(t.n10, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n10, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n10, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n10, 17 /*marginTop*/, staticObj[5]);
            direct_1.setEvent(t.n10, "pointerclick", $event => {
                let r = w.modifyA1($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n10, "修改a1, 监听器1将修改a3; 监听器2将修改a4, a5");
            t.n11 = direct_1.createSpan();
            direct_1.setStyle(t.n11, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n11, 1 /*height*/, staticObj[3]);
            direct_1.setStyle(t.n11, 46 /*backgroundColor*/, staticObj[4]);
            direct_1.setStyle(t.n11, 17 /*marginTop*/, staticObj[5]);
            direct_1.setEvent(t.n11, "pointerclick", $event => {
                let r = w.modifyA2($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n11, "修改a2, 监听器2将修改a4, a5");
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n5, t.n3);
            direct_1.append(t.n6, t.n3);
            direct_1.append(t.n7, t.n3);
            direct_1.append(t.n8, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n10, t.n9);
            direct_1.append(t.n11, t.n9);
            direct_1.append(t.n9, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            if (dirty0 & 1) direct_1.setText(t.n4, "a1:" + w.a1);
            if (dirty0 & 2) direct_1.setText(t.n5, "a2:" + w.a2);
            if (dirty0 & 4) direct_1.setText(t.n6, "a3:" + w.a3);
            if (dirty0 & 8) direct_1.setText(t.n7, "a4:" + w.a4);
            if (dirty0 & 16) direct_1.setText(t.n8, "a5:" + w.a5);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});