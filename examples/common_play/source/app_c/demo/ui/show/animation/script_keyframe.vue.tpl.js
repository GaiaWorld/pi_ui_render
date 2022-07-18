_$pi.define("app_c/demo/ui/show/animation/script_keyframe.vue.tpl", ["require", "exports", "module", "pi_gui/widget/direct"], function (require, exports, module, direct_1) {
    "use strict";

    exports.BW2 = void 0;
    const staticObj = [[0, 700], [0, 400], [0, 100], [1, 0, 0, 1], [0, 50], [0, 0.52, 1, 1], [0, 0.33, 1, 1]];
    class BW2 {
        constructor(widget, ctx) {
            this.w = widget;
        }
        c() {
            let t = this,
                w = t.w;
            t.n2 = direct_1.createDiv();
            direct_1.setStyle(t.n2, 61 /*display*/, 0);
            direct_1.setStyle(t.n2, 6 /*position*/, 1);
            direct_1.setStyle(t.n2, 0 /*width*/, staticObj[0]);
            direct_1.setStyle(t.n2, 1 /*height*/, staticObj[1]);
            direct_1.setStyle(t.n2, 21 /*flexDirection*/, 0);
            direct_1.setStyle(t.n2, 22 /*flexWrap*/, 1);
            direct_1.extendAttr(t.n2, w, null, true);
            t.n3 = direct_1.createDiv();
            direct_1.setAttr(t.n3, "ref", "animTarget");
            direct_1.setStyle(t.n3, 0 /*width*/, staticObj[2]);
            direct_1.setStyle(t.n3, 1 /*height*/, staticObj[2]);
            direct_1.setStyle(t.n3, 46 /*backgroundColor*/, staticObj[3]);
            t.n4 = direct_1.createDiv();
            direct_1.setStyle(t.n4, 22 /*flexWrap*/, 1);
            t.n5 = direct_1.createSpan();
            direct_1.setStyle(t.n5, 1 /*height*/, staticObj[4]);
            direct_1.setStyle(t.n5, 46 /*backgroundColor*/, staticObj[5]);
            direct_1.setEvent(t.n5, "pointerclick", $event => {
                let r = w.playFrameAnimation($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n5, "点击播放帧动画");
            t.n6 = direct_1.createSpan();
            direct_1.setStyle(t.n6, 1 /*height*/, staticObj[4]);
            direct_1.setStyle(t.n6, 46 /*backgroundColor*/, staticObj[6]);
            direct_1.setEvent(t.n6, "pointerclick", $event => {
                let r = w.playCustomAnimation($event);
                $event && typeof $event === "object" && ($event.stopPropagation = !r);
                return r;
            });
            direct_1.setText(t.n6, "点击播放自定义动画");
            return this.n2;
        }
        m(target, anchor) {
            let t = this,
                w = t.w;
            direct_1.append(t.n3, t.n2);
            direct_1.append(t.n5, t.n4);
            direct_1.append(t.n6, t.n4);
            direct_1.append(t.n4, t.n2);
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