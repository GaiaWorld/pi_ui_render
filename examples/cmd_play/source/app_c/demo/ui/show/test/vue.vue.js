_$pi.define("app_c/demo/ui/show/test/vue.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./vue.vue.tpl", "pi_gui/widget/direct", "pi_common/ui/main_root"], function (require, exports, module, direct_1, vue_vue_tpl_1, direct_2, main_root_1) {
    "use strict";

    exports.initMeta = exports.Unknown = exports.TestModel = exports.TestSlotWithDefaultMult = exports.TestSlotWithDefault = exports.TestSlot = exports.TestForWithKey = exports.TestFor = exports.TestShow = exports.TestIf = exports.TestSingleInner1 = exports.TestSingleInner = void 0;
    class TestSingle {
        constructor() {
            this.clazzName = "e2";
            this.className = "size100";
            this.clazzName1 = "e2";
            this.className1 = "size100";
            this.slot = 1;
            this.inputValue = "123456";
        }
        create() {
            this.color = "rgba(255,0,0,1)";
        }
        animationStart(e) {
            console.log("animationStart", e);
        }
        animationIter(e) {
            console.log("animationIter", e);
        }
        animationEnd(e) {
            console.log("animationEnd", e);
        }
        changeSlot() {
            console.log("修改slot");
            this.slot = this.slot + 1;
        }
        changeClazzName1() {
            if (this.clazzName1 === "e2") {
                this.clazzName1 = "e4";
            } else {
                this.clazzName1 = "e2";
            }
            console.log("changeClazzName1", this.clazzName1, "但clazzName1为不可变变量，界面不变化");
        }
        changeClazzName() {
            if (this.clazzName === "e2") {
                this.clazzName = "e4";
            } else {
                this.clazzName = "e2";
            }
            console.log("changeClazzName", this.clazzName);
        }
        destroy() {
            console.log("销毁组件", this.constructor.name);
        }
        close() {
            main_root_1.close(this);
        }
    }
    exports.default = TestSingle;
    class TestSingleInner {
        constructor() {
            this.clazzName = "e2";
            this.className = "size100";
            this.height = 300;
        }
        create() {
            this.color = "rgba(255,0,0,1)";
        }
        changeHeight() {
            this.height += 30;
            // this["_$block"].p();
        }
        destroy() {
            console.log("销毁组件", this.constructor.name);
        }
    }
    exports.TestSingleInner = TestSingleInner;
    class TestSingleInner1 {
        constructor() {
            this.clazzName = "e2";
            this.className = "size100";
        }
        create() {
            this.color = "rgba(255,0,0,1)";
        }
        say(param) {
            alert(param);
            direct_2.emit(this, "ev-use-evt", "TestSingleInner1 send");
        }
        destroy() {
            console.log("销毁组件", this.constructor.name);
        }
    }
    exports.TestSingleInner1 = TestSingleInner1;
    class TestIf {
        create() {
            this.show = "blue";
            this.height = 50;
        }
        changeIf() {
            if (this.height == 50) {
                this.height = 100;
                console.log("分支不变，尺寸变为100");
            } else {
                this.height = 50;
                if (this.show === "blue") {
                    this.show = "red";
                } else {
                    this.show = "blue";
                }
                console.log("分支改变");
            }
            // this["_$block"].p();
        }
        destroy() {
            console.log("销毁组件", this.constructor.name);
        }
    }
    exports.TestIf = TestIf;
    class TestShow {
        changeShow() {
            this.show = !this.show;
        }
    }
    exports.TestShow = TestShow;
    class TestFor {
        constructor() {
            this.count = 0;
        }
        create() {
            this.setDate();
        }
        setDate() {
            let count = Math.round(Math.random() * 9) + 1;
            this.arr = [];
            for (let i = 1; i <= count; i++) {
                this.arr.push(i);
            }
            let roundIndex = Math.round(Math.random() * (count - 1));
            let roundIndex1 = Math.round(Math.random() * (count - 1));
            let t = this.arr[roundIndex];
            this.arr[roundIndex] = this.arr[roundIndex1];
            this.arr[roundIndex1] = t;
            console.log("arr:", this.arr, "len:", this.arr.length);
        }
        change() {
            this.setDate();
            // this["_$block"].p();
        }
        destroy() {
            console.log("销毁组件", this.constructor.name);
        }
    }
    exports.TestFor = TestFor;
    class TestForWithKey extends TestFor {
        destroy() {
            console.log("销毁组件", this.constructor.name);
        }
    }
    exports.TestForWithKey = TestForWithKey;
    class TestSlot {
        constructor() {
            this.name = "TestSlot";
            this.count = 0;
        }
        change() {
            this.name = "TestSlot" + this.count++;
            console.log("TestSlot 修改：", this.name);
        }
        destroy() {
            console.log("销毁组件", this.constructor.name);
        }
    }
    exports.TestSlot = TestSlot;
    class TestSlotWithDefault {
        constructor() {
            this.name = "TestSlotWithDefault";
            this.count = 0;
        }
        change() {
            this.name = "TestSlotWithDefault" + this.count++;
            console.log("TestSlotWithDefault 修改：", this.name);
        }
        destroy() {
            console.log("销毁组件", this.constructor.name);
        }
    }
    exports.TestSlotWithDefault = TestSlotWithDefault;
    class TestSlotWithDefaultMult {
        constructor() {
            this.name = "TestSlotWithDefaultMult";
            this.count = 0;
        }
        change() {
            this.name = "TestSlotWithDefaultMult" + this.count++;
            console.log("TestSlotWithDefaultMult 修改：", this.name);
        }
        destroy() {
            console.log("销毁组件", this.constructor.name);
        }
    }
    exports.TestSlotWithDefaultMult = TestSlotWithDefaultMult;
    // 测试组件的双向绑定
    class TestModel {}
    exports.TestModel = TestModel;
    TestModel.model = {
        prop: "name",
        event: "change"
    };
    class Unknown {}
    exports.Unknown = Unknown;
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/test/vue.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/test/vue.vue.wcss",
            _$cssHash = 2592261505;
        TestSingleInner["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW25, cssHash: _$cssHash };
        TestSingleInner1["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW27, cssHash: _$cssHash };
        TestIf["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW31, cssHash: _$cssHash };
        TestShow["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW34, cssHash: _$cssHash };
        TestFor["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW38, cssHash: _$cssHash };
        TestForWithKey["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW43, cssHash: _$cssHash };
        TestSlot["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW47, cssHash: _$cssHash };
        TestSlotWithDefault["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW50, cssHash: _$cssHash };
        TestSlotWithDefaultMult["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW54, cssHash: _$cssHash };
        Unknown["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW59, cssHash: _$cssHash };
        TestModel["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW61, cssHash: _$cssHash };
        TestSingle["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: vue_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(TestSingle, ["color", "clazzName", "className", "clazzName1", "className1", "inputValue", "slot"]);
    direct_1.defineAccessors(TestSingleInner, ["clazzName", "className", "color", "height"]);
    direct_1.defineAccessors(TestSingleInner1, ["color", "clazzName", "className"]);
    direct_1.defineAccessors(TestIf, ["show", "height"]);
    direct_1.defineAccessors(TestShow, ["show"]);
    direct_1.defineAccessors(TestFor, ["arr"]);
    direct_1.defineAccessors(TestForWithKey, ["arr"]);
    direct_1.defineAccessors(TestSlot, ["name"]);
    direct_1.defineAccessors(TestSlotWithDefault, ["name"]);
    direct_1.defineAccessors(TestSlotWithDefaultMult, ["name"]);
    direct_1.defineAccessors(TestModel, ["name"]);
    direct_1.addField(TestFor, ['count']);
    direct_1.addField(TestSlot, ['count']);
    direct_1.addField(TestSlotWithDefault, ['count']);
    direct_1.addField(TestSlotWithDefaultMult, ['count']);
});