_$pi.define("app_c/demo/ui/show/animation/script_keyframe.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./script_keyframe.vue.tpl", "pi_gui/engine/animation_tools", "pi_gui/engine/tools", "pi_gui/widget/direct", "pi_utils/util/frame_mgr"], function (require, exports, module, direct_1, script_keyframe_vue_tpl_1, animation_tools_1, tools_1, direct_2, frame_mgr_1) {
    "use strict";

    exports.initMeta = void 0;
    class ScriptKeyFrame {
        constructor() {
            this.opacity = 1;
            this.opacityStep = -0.016;
            this.animStart = () => {
                console.warn('start!!!');
            };
            this.animItera = looptime => {
                console.warn('animItera!!!', looptime);
            };
            this.animEnd = () => {
                console.warn('animEnd!!!');
            };
        }
        /**
         * 播放帧动画
         */
        playFrameAnimation() {
            const animNode = direct_2.ref(this, "animTarget");
            // 创建运行时动画
            const runtimeAnimation = animation_tools_1.AnimeTools.initRuntimeAnimation(anim_cmd2, anim_frame2);
            // 动画状态监听
            animNode.style.addAnimListener(anim_cmd2.name, 'start', this.animStart);
            animNode.style.addAnimListener(anim_cmd2.name, 'loop', this.animItera);
            animNode.style.addAnimListener(anim_cmd2.name, 'end', this.animEnd);
            // 启动动画
            animNode.style.addAnimation(runtimeAnimation);
        }
        /**
         * 除了用帧数据描述动画，也可以自行控制真实节点的style，来模拟动画效果
         * 不能提前预知动画的帧数据，可以采取这种方式来控制动画
         * 注意，当需要自行控制动画时，不要采用改变prop的数据，使widget不断paint来获得动画效果，这样做十分消耗性能
         */
        playCustomAnimation() {
            if (this.loop) {
                return;
            }
            const animNode = direct_2.ref(this, "animTarget");
            let vdocument = direct_2.document(this);
            this.loop = () => {
                if (this.opacity == 1) {
                    this.opacityStep = -0.016;
                } else if (this.opacity == 0) {
                    this.opacityStep = 0.016;
                }
                this.opacity += this.opacityStep;
                this.opacity = Math.max(Math.min(this.opacity, 1), 0);
                vdocument.applyStyle(animNode.style, "opacity", this.opacity);
            };
            frame_mgr_1.getGlobal().setPermanent(this.loop);
        }
        detach() {
            if (this.loop) {
                frame_mgr_1.getGlobal().clearPermanent(this.loop);
                this.loop = undefined;
            }
        }
    }
    exports.default = ScriptKeyFrame;
    /**
     * 动画执行配置数据
     */
    const anim_cmd2 = new tools_1.AnimationCmd('scale_anim');
    anim_cmd2.iteration = -1;
    anim_cmd2.duration = 5000;
    anim_cmd2.fillMode = 'forwards';
    /**
     * 关键帧数据
     */
    const anim_frame2 = {
        name: 'scale_anim',
        attrs: [{
            /**
             * 属性名
             */
            key: animation_tools_1.FrameClassKeys.transform,
            /**
             * 属性值配置
             */
            data: [[0, tools_1.Tools.readTransform(tools_1.Tools.transform("scale(1,1)"))], [0.5, tools_1.Tools.readTransform(tools_1.Tools.transform("scale(2,2)"))], [1.0, tools_1.Tools.readTransform(tools_1.Tools.transform("scale(1,1)"))]]
        }]
    };
    exports.initMeta = () => {
        let _$tpl = "app_c/demo/ui/show/animation/script_keyframe.vue.tpl.ts",
            _$cssPath = "app_c/demo/ui/show/animation/script_keyframe.vue.wcss",
            _$cssHash = 765599624;
        ScriptKeyFrame["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: script_keyframe_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.addField(ScriptKeyFrame, ['loop', 'opacity', 'opacityStep', 'animStart', 'animItera', 'animEnd']);
});