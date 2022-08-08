_$pi.define("app_c/demo/ui/show/base/opacity.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[1, 1], [0, 200], [0.2, 0.2, 0.35, 1], [0, 64], [1, 1, 1, 1], [0, 120], [0, 10], [0, 20], [0.5, 0.5, 0.5, 1]];
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
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[1]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[2]);
            direct_1.setStyle(t.n3, 65 /*opacity*/, w.opacity);
            t.n4 = direct_1.createImg();
            direct_1.setAttr(t.n4, "src", "app_c/demo/ui/images/act_04.png");
            direct_1.setStyle(t.n4, 0 /*width*/, staticObj[3]);
            direct_1.setStyle(t.n4, 1 /*height*/, staticObj[3]);
            t.n5 = direct_1.createDiv();
            t.n6 = direct_1.createSpan();
            direct_1.setStyle(t.n6, 30 /*color*/, staticObj[4]);
            direct_1.setStyle(t.n6, 0 /*width*/, staticObj[5]);
            direct_1.setStyle(t.n6, 20 /*marginLeft*/, staticObj[6]);
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n6, 42 /*fontSize*/, 20);
            direct_1.setText(t.n6, "opacity:" + w.opacity);
            t.n7 = direct_1.createSpan();
            direct_1.setStyle(t.n7, 46 /*backgroundColor*/, staticObj[8]);
            direct_1.setStyle(t.n7, 20 /*marginLeft*/, staticObj[6]);
            direct_1.setStyle(t.n7, 1 /*height*/, staticObj[7]);
            direct_1.setStyle(t.n7, 42 /*fontSize*/, 20);
            direct_1.setEvent(t.n7, "pointerclick", $event => {
                let r = w.change($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n7, "opacity-0.1");
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n4, t.n3);
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n6, t.n5);
            direct_1.append(t.n7, t.n5);
            direct_1.append(t.n5, t.n2);
            direct_1.insertBefore(t.n2, target, anchor);
        }
        p() {
            let t = this,
                w = t.w;
            let { dirty0 } = w._$info;
            direct_1.extendAttr(t.n2, w);
            if (dirty0 & 1) direct_1.setStyle(t.n3, 65 /*opacity*/, w.opacity);
            if (dirty0 & 1) direct_1.setText(t.n6, "opacity:" + w.opacity);
            return this.n2;
        }
    }
    exports.BW2 = BW2;
});