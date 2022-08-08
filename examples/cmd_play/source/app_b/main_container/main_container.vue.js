_$pi.define("app_b/main_container/main_container.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./main_container.vue.tpl", "app_a/util/bezier", "pi_gui/widget/direct", "pi_gui/engine/math_tools", "../main/main.vue", "app_c/rank/client/rank.vue", "pi_utils/util/logger", "pi_utils/util/event", "pi_sys/index", "pi_gui/engine/animation_tools", "pi_gui/engine/tools", "app_b/scene/scene", "pi_common/ui/main_root", "app_b/meteor/main", "app_b/scene/scene_data", "app_b/scene/main", "app_a/widget/tips/tips.vue"], function (require, exports, module, direct_1, main_container_vue_tpl_1, bezier_1, direct_2, math_tools_1, main_vue_1, rank_vue_1, logger_1, event_1, index_1, animation_tools_1, tools_1, scene_1, main_root_1, main_1, scene_data_1, main_2, tips_vue_1) {
    "use strict";

    exports.initMeta = exports.ScreenEvent = exports.SlideType = exports.initSceneAndMeteor = exports.toggleBottomMenu = exports.WIDGET_MAP = exports.SCREEN_HANDLER = void 0;
    const { logV, logD, logI, logW, logE } = logger_1.getLogger(module.name, 'app');
    math_tools_1.MathTools.register('easeOutCustom', bezier_1.easingFunc);
    math_tools_1.MathTools.register('easeOutCubic', bezier_1.bezier(0.215, 0.61, 0.355, 1));
    math_tools_1.MathTools.register('collectEasing', bezier_1.collectEasingFunc);
    math_tools_1.MathTools.register('slowFunc', bezier_1.slowFunc);
    math_tools_1.MathTools.register('treasureEasing', bezier_1.bezier(0.4, 0, 0.9, 0.3));
    math_tools_1.MathTools.register('testFunc', x => -4 * Math.pow(x, 2) + 4 * x);
    exports.SCREEN_HANDLER = new event_1.HandlerMap();
    exports.WIDGET_MAP = new Map();
    let w;
    class MainContainer {
        constructor() {
            this.widget1 = main_vue_1.default;
            this.widget2 = null;
            this.showMenu = true;
            this.curWidget = 'widget1';
            this.nextWidget = 'widget2';
            this.bottomMenu = [{
                image: 'main',
                name: '场景',
                selected: true,
                widget: main_vue_1.default,
                canUse: () => true
            }, {
                image: 'treasure',
                name: '宝箱',
                selected: false,
                widget: null,
                canUse: () => false
            }, {
                image: 'animalmap',
                name: '战斗',
                selected: false,
                widget: null,
                canUse: () => false
            }, {
                image: 'rank',
                name: '排行',
                selected: false,
                widget: rank_vue_1.default,
                canUse: () => true
            }, {
                image: 'science',
                name: '其它',
                selected: false,
                widget: null,
                canUse: () => false
            }];
            this.prevevt = false; // 是否阻止事件
            this.animeCounter = 0; // 切换动画完成数
            this.vaildMenux = []; // 底部菜单X有效范围
            this.vaildMenuY = []; // 底部菜单Y有效范围
            this.screenWidth = 750;
            this.lastMenu = 0;
            this.getAdaptiveResult = () => {
                const adaptiveResult = index_1.PISYS.Env.get('adaptiveResult');
                const xMax = adaptiveResult.renderSize.width - 15;
                const xMin = 15;
                const yMax = adaptiveResult.renderSize.height;
                const yMin = adaptiveResult.renderSize.height - 135;
                const sepra = (xMax - xMin - 144 * 5) / 4;
                const vaildMenux = [];
                let x1 = xMin;
                let x2 = x1 + 144;
                for (let i = 0; i < 5; i++) {
                    vaildMenux.push([x1, x2]);
                    x1 = x2 + sepra;
                    x2 = 144 + x1;
                }
                this.vaildMenux = vaildMenux;
                this.vaildMenuY = [yMin, yMax];
                this.screenWidth = adaptiveResult.renderSize.width;
            };
            this.getTouchResult = e => {
                if (!this.vaildMenux || !this.vaildMenuY) return -1;
                const [yMin, yMax] = this.vaildMenuY;
                if (e.clientY > yMax || e.clientY < yMin) return -1; // 超出菜单按钮范围
                for (let i = 0; i < 5; i++) {
                    if (e.clientX >= this.vaildMenux[i][0] && e.clientX <= this.vaildMenux[i][1]) {
                        return i;
                    }
                }
                return -1;
            };
            // 开始滑动
            this.popHandler = args => {
                if (this.prevevt) {
                    return;
                }
                if (!args[1]) {
                    return logE(`can't pop empty widget: ${args[1]}`);
                }
                const outName = this[this.curWidget];
                if (args[1] === outName) {
                    return logE(`can't pop same widget: ${args[1]}`);
                }
                // logW('now slide start, wait widget paint');
                this.prevevt = true;
                this.direct = args[0];
                if (this[this.nextWidget] === args[1]) {
                    return exports.SCREEN_HANDLER.notify(ScreenEvent.WidgetPainted, [args[1]]);
                }
                this[this.nextWidget] = args[1];
            };
            // 滑动中
            this.transformHandler = args => {
                if (args[0] === this[this.nextWidget]) {
                    const outElement = direct_2.getDomNode(direct_2.findElementByAttr(this, 'id', this.curWidget));
                    const inElement = direct_2.getDomNode(direct_2.findElementByAttr(this, 'id', this.nextWidget));
                    if (!outElement || !inElement) {
                        return logE(`can't find ${args[0]} or current screen is undefind,
                mostly because build widget tree failed, please clear site data, and try again`);
                    }
                    if (this.direct === SlideType.SlideLeft) {
                        this.translateLeftStart(outElement, this[this.curWidget], inElement, this[this.nextWidget]);
                    } else {
                        this.translateRightStart(outElement, this[this.curWidget], inElement, this[this.nextWidget]);
                    }
                }
            };
            // 滑动结束
            this.slideEndHandler = args => {
                if (args[0] === this.widget1 || args[0] === this.widget2) {
                    this.animeCounter++;
                    if (this.animeCounter >= 2) {
                        const tmp = this.curWidget;
                        this.curWidget = this.nextWidget;
                        this.nextWidget = tmp;
                        this[this.nextWidget] = null;
                        this.animeCounter = 0;
                        this.prevevt = false;
                        // logW('slide end update prop', this.currWidget, this.props[this.currWidget], this.nextWidget, this.props[this.nextWidget]);
                    }
                }
            };
            // 往左滑
            this.translateLeftStart = (outElement, outName, inElement, inName) => {
                this.checkBeforeSlideOut(outName);
                this.slideRightIn(inElement, inName);
                this.slideLeftOut(outElement, outName);
            };
            // 往右滑
            this.translateRightStart = (outElement, outName, inElement, inName) => {
                this.checkBeforeSlideOut(outName);
                this.slideLeftIn(inElement, inName);
                this.slideRightOut(outElement, outName);
            };
            // 生成动画
            this.generateCmd = animeName => {
                const init_anim_cmd = new tools_1.AnimationCmd(animeName);
                init_anim_cmd.iteration = 1;
                init_anim_cmd.duration = slideTime;
                init_anim_cmd.fillMode = 'both';
                init_anim_cmd.timingFunction = timeFunc;
                return init_anim_cmd;
            };
            // 生成关键帧
            this.generateKeyfames = (s, e) => {
                return {
                    name: 'openAnim',
                    attrs: [{
                        key: animation_tools_1.FrameClassKeys.transform,
                        data: [[0, tools_1.Tools.readTransform(tools_1.Tools.analy('transform', `translateX(${s}px)`))], [1, tools_1.Tools.readTransform(tools_1.Tools.analy('transform', `translateX(${e}px)`))]]
                    }]
                };
            };
            // 场景移动
            this.connSceneMove = (widName, root, direct) => {
                const sceneName = exports.WIDGET_MAP.get(widName)._sceneName;
                if (!sceneName) return;
                if (!root.style.hasOwnProperty('animeTabLastTime')) {
                    Object.defineProperty(root.style, 'animeTabLastTime', {
                        get: function () {
                            return this._animatable.lastTime;
                        }
                    });
                }
                if (direct === SlideDirect.LeftIn) {
                    scene_1.SceneAnimation.slideLeftIn(root.style.animeTabLastTime - 17);
                } else if (direct === SlideDirect.LeftOut) {
                    scene_1.SceneAnimation.slideLeftOut(root.style.animeTabLastTime + 34);
                } else if (direct === SlideDirect.RightIn) {
                    scene_1.SceneAnimation.slideRightIn(root.style.animeTabLastTime - 17);
                } else {
                    scene_1.SceneAnimation.slideRightOut(root.style.animeTabLastTime + 34);
                }
            };
            // 滑动前检查
            this.checkBeforeSlideOut = outName => {
                const outData = exports.WIDGET_MAP.get(outName);
                const wid = outData._widget;
                if (!wid) return;
                wid.beforeSlideOut && wid.beforeSlideOut();
            };
            // 激活场景
            this.activeScene = widName => {
                const { IndexScene } = index_1.PISYS.Module.requireSync('app_b/scene/main');
                const sceneName = exports.WIDGET_MAP.get(widName)._sceneName;
                if (sceneName) {
                    IndexScene.active(); // 目前只有主场景
                }
            };
            // 暂停场景
            this.pauseScene = widName => {
                const { IndexScene } = index_1.PISYS.Module.requireSync('app_b/scene/main');
                const sceneName = exports.WIDGET_MAP.get(widName)._sceneName;
                if (sceneName) {
                    IndexScene.pause(); // 目前只有主场景
                }
            };
        }
        getWidget(name) {
            var _a;
            return (_a = this.bottomMenu.find(v => v.name === name)) === null || _a === void 0 ? void 0 : _a.widget;
        }
        create() {
            w = this;
            exports.SCREEN_HANDLER.add(ScreenEvent.PopSlide, this.popHandler);
            exports.SCREEN_HANDLER.add(ScreenEvent.WidgetPainted, this.transformHandler);
            exports.SCREEN_HANDLER.add(ScreenEvent.SlideEnd, this.slideEndHandler);
            this.getAdaptiveResult();
        }
        destroy() {
            w = null;
            exports.SCREEN_HANDLER.remove(ScreenEvent.PopSlide, this.popHandler);
            exports.SCREEN_HANDLER.remove(ScreenEvent.WidgetPainted, this.transformHandler);
            exports.SCREEN_HANDLER.remove(ScreenEvent.SlideEnd, this.slideEndHandler);
        }
        onDown(i) {
            if (this.bottomMenu[i].selected) {
                return;
            }
            this.bottomMenu.map((v, index) => v.selected = i === index);
        }
        upDefault(e) {
            const i = this.getTouchResult(e);
            if (i !== -1 && this.bottomMenu[i].canUse()) {
                this.openinterface(i);
            } else {
                this.onDown(this.lastMenu);
            }
        }
        sliding(e) {
            const i = this.getTouchResult(e);
            if (i !== -1) {
                this.onDown(i);
            } else {
                this.onDown(this.lastMenu);
            }
        }
        openinterface(i) {
            if (!this.bottomMenu[i].canUse()) {
                return tips_vue_1.showTips('尚未解锁');
            }
            for (let i = 0; i < this.bottomMenu.length; i++) {
                this.bottomMenu[i].selected = false;
            }
            this.bottomMenu[i].selected = true;
            this.openinterfacePop(i); // 进入不同功能页面
        }
        openinterfacePop(i) {
            const currWidgetIdx = this.bottomMenu.findIndex(v => v.widget === this[this.curWidget]);
            if (i === currWidgetIdx) {
                return;
            }
            const nextScreen = this.bottomMenu[i].widget;
            const slideTyle = currWidgetIdx < i ? SlideType.SlideLeft : SlideType.SlideRight;
            exports.SCREEN_HANDLER.notify(ScreenEvent.PopSlide, [slideTyle, nextScreen, {}]);
        }
        // 左入
        slideLeftIn(node, widName) {
            const init_anim_cmd = this.generateCmd('openAnim');
            const keyFrames = this.generateKeyfames(-this.screenWidth, 0);
            const runtimeAnimation = animation_tools_1.AnimeTools.initRuntimeAnimation(init_anim_cmd, keyFrames);
            node.style.addAnimListener(init_anim_cmd.name, 'end', () => {
                exports.SCREEN_HANDLER.notify(ScreenEvent.SlideEnd, [widName]);
            });
            // 启动动画
            this.activeScene(widName);
            node.style.addAnimation(runtimeAnimation);
            this.connSceneMove(widName, node, SlideDirect.LeftIn);
        }
        // 左出
        slideLeftOut(node, widName) {
            // 动画执行配置数据;
            const init_anim_cmd = this.generateCmd('closeAnim');
            const keyFrames = this.generateKeyfames(0, -this.screenWidth);
            const runtimeAnimation = animation_tools_1.AnimeTools.initRuntimeAnimation(init_anim_cmd, keyFrames);
            node.style.addAnimListener(init_anim_cmd.name, 'end', () => {
                exports.SCREEN_HANDLER.notify(ScreenEvent.SlideEnd, [widName]);
                this.pauseScene(widName);
            });
            // 启动动画
            node.style.addAnimation(runtimeAnimation);
            this.connSceneMove(widName, node, SlideDirect.LeftOut);
        }
        // 右入
        slideRightIn(node, widName) {
            const init_anim_cmd = this.generateCmd('openAnim');
            const keyFrames = this.generateKeyfames(this.screenWidth, 0);
            const runtimeAnimation = animation_tools_1.AnimeTools.initRuntimeAnimation(init_anim_cmd, keyFrames);
            node.style.addAnimListener(init_anim_cmd.name, 'end', () => {
                exports.SCREEN_HANDLER.notify(ScreenEvent.SlideEnd, [widName]);
            });
            // 启动动画
            this.activeScene(widName);
            node.style.addAnimation(runtimeAnimation);
            this.connSceneMove(widName, node, SlideDirect.RightIn);
        }
        // 右出
        slideRightOut(node, widName) {
            // 动画执行配置数据;
            const init_anim_cmd = this.generateCmd('closeAnim');
            const keyFrames = this.generateKeyfames(0, this.screenWidth);
            const runtimeAnimation = animation_tools_1.AnimeTools.initRuntimeAnimation(init_anim_cmd, keyFrames);
            node.style.addAnimListener(init_anim_cmd.name, 'end', () => {
                exports.SCREEN_HANDLER.notify(ScreenEvent.SlideEnd, [widName]);
                this.pauseScene(widName);
            });
            // 启动动画
            node.style.addAnimation(runtimeAnimation);
            this.connSceneMove(widName, node, SlideDirect.RightOut);
        }
    }
    exports.default = MainContainer;
    exports.toggleBottomMenu = (showMenu, immediately = false) => {
        if (!w) return;
        const rootNode = direct_2.getDomNode(w);
        const vdocument = main_root_1.getRoot().document;
        if (rootNode.childNodes[2]) {
            if (!immediately) {
                vdocument.applyStyle(rootNode.childNodes[2].style, 'animation', direct_2.createRunTimeAnimation(`${showMenu ? 'enter' : 'leave'} 0.5s linear 0 1 none none;`, w));
            } else {
                vdocument.applyStyle(rootNode.childNodes[2].style, 'transform', `translateY(${showMenu ? 0 : 135}px)`);
            }
        }
    };
    const openMainContainer = () => {
        try {
            main_1.initMeteor();
            main_root_1.pop(MainContainer);
        } catch (error) {
            logD('error: ', error);
        }
        scene_data_1.bus.remove(scene_data_1.SceneEvent.SCENE_LOADED, openMainContainer);
    };
    function initSceneAndMeteor() {
        try {
            scene_data_1.bus.add(scene_data_1.SceneEvent.SCENE_LOADED, openMainContainer);
            main_2.IndexScene.init();
        } catch (error) {
            logD('error: ', JSON.stringify(error), error.message);
        }
    }
    exports.initSceneAndMeteor = initSceneAndMeteor;
    // 整体滑动方向
    var SlideType;
    (function (SlideType) {
        SlideType[SlideType["SlideRight"] = 0] = "SlideRight";
        SlideType[SlideType["SlideLeft"] = 1] = "SlideLeft";
    })(SlideType = exports.SlideType || (exports.SlideType = {}));
    // 时间
    var ScreenEvent;
    (function (ScreenEvent) {
        ScreenEvent["PopSlide"] = "PopSlide";
        ScreenEvent["WidgetPainted"] = "WidgetPainted";
        ScreenEvent["SlideEnd"] = "SlideEnd";
    })(ScreenEvent = exports.ScreenEvent || (exports.ScreenEvent = {}));
    // 记录单个widget滑动方向
    var SlideDirect;
    (function (SlideDirect) {
        SlideDirect[SlideDirect["LeftIn"] = 0] = "LeftIn";
        SlideDirect[SlideDirect["LeftOut"] = 1] = "LeftOut";
        SlideDirect[SlideDirect["RightIn"] = 2] = "RightIn";
        SlideDirect[SlideDirect["RightOut"] = 3] = "RightOut";
    })(SlideDirect || (SlideDirect = {}));
    // 滑动时间
    const slideTime = 300;
    // 滑动曲线
    const timeFunc = 'easeOutCustom';
    exports.initMeta = () => {
        let _$tpl = "app_b/main_container/main_container.vue.tpl.ts",
            _$cssPath = "app_b/main_container/main_container.vue.wcss",
            _$cssHash = 3344951921;
        MainContainer["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: main_container_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(MainContainer, ["widget1", "widget2", "showMenu", "bottomMenu"]);
    direct_1.addField(MainContainer, ['curWidget', 'nextWidget', 'props1', 'props2', 'prevevt', 'direct', 'animeCounter', 'vaildMenux', 'vaildMenuY', 'screenWidth', 'lastMenu']);
});