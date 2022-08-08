_$pi.define("app_c/demo/ui/show/base/filter.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./filter.vue.tpl"], function (require, exports, module, direct_1, filter_vue_tpl_1) {
    "use strict";

    exports.initMeta = void 0;
    class HsiWidget {
        constructor() {
            this.h = 0; // -180~180
            this.s = 0; // -100~100
            this.i = 0; // -100~100
            // 0~360 给图像应用色相旋转。"angle"一值设定图像会被调整的色环角度值。值为0deg，则图像无变化。若值未设置，默认值是0deg。该值虽然没有最大值，超过360deg的值相当于又绕一圈。
            this.hueRotate = 0;
            // 0~+Infinity 转换图像饱和度。值定义转换的比例。值为0%则是完全不饱和，值为100%则图像无变化。其他值，则是效果的线性乘子。超过100%的值是允许的，则有更高的饱和度。 若值未设置，值默认是1。
            this.saturate = 100;
            // 0~+Infinity  给图片应用一种线性乘法，使其看起来更亮或更暗。如果值是0%，图像会全黑。值是100%，则图像无变化。其他的值对应线性乘数效果。值超过100%也是可以的，图像会比原来更亮。如果没有设定值，默认是1。
            this.brightness = 100;
            // 将图像转换为灰度图像。值定义转换的比例。值为100%则完全转为灰度图像，值为0%图像无变化。值在0%到100%之间，则是效果的线性乘子。若未设置，值默认是0；
            this.grayscale = 0;
            this.blur = 0; // 模糊半径
            // 样式中声明filter，切换样式
            this.cssList = ["brightness", "gray", "hs", "hue", "saturate"];
            this.cssListi = 0;
        }
        changeh() {
            this.h = addValue(this.h);
        }
        changes() {
            this.s = addValue1(this.s);
        }
        changei() {
            this.i = addValue1(this.i);
        }
        changeh1() {
            if (this.hueRotate >= 360) {
                this.hueRotate = 0;
            } else {
                this.hueRotate = this.hueRotate + 10;
            }
        }
        changes1() {
            if (this.saturate >= 200) {
                this.saturate = 0;
            } else {
                this.saturate = this.saturate + 10;
            }
        }
        changei1() {
            if (this.brightness >= 200) {
                this.brightness = 0;
            } else {
                this.brightness = this.brightness + 10;
            }
        }
        changeg1() {
            if (this.grayscale > 100) {
                this.grayscale = 0;
            } else {
                this.grayscale = this.grayscale + 10;
            }
        }
        changegcss() {
            this.cssListi += 1;
            if (this.cssListi >= this.cssList.length) {
                this.cssListi = 0;
            }
        }
        changeBlur() {
            if (this.blur < 5) {
                this.blur++;
            } else {
                this.blur = 0;
            }
        }
    }
    exports.default = HsiWidget;
    const addValue = v => {
        if (v >= 180) {
            return -180;
        } else {
            return v + 10;
        }
    };
    const addValue1 = v => {
        if (v >= 100) {
            return -100;
        } else {
            return v + 10;
        }
    };
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/base/filter.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/base/filter.vue.wcss",
            _$cssHash = 3323958850;
        HsiWidget["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: filter_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(HsiWidget, ["h", "s", "i", "hueRotate", "saturate", "brightness", "grayscale", "cssList", "cssListi", "blur"]);
});