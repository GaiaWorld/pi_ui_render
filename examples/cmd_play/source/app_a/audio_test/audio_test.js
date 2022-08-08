_$pi.define("app_a/audio_test/audio_test", ["require", "exports", "module", "pi_gui/widget/widget", "pi_gui/widget/forelet", "pi_utils/res/sound", "pi_utils/res/sound", "pi_common/ui/main_root", "pi_sys/media/inner_audio_impl", "pi_utils/res/res_mgr", "pi_utils/res/audio_mgr"], function (require, exports, module, widget_1, forelet_1, sound_1, sound_2, main_root_1, inner_audio_impl_1, res_mgr_1, audio_mgr_1) {
    "use strict";

    exports.AudioTest = exports.forelet = void 0;
    exports.forelet = new forelet_1.Forelet();
    let innerAudio;
    let soundVolume = 1;
    let bgmVolume = 1;
    class AudioTest extends widget_1.Widget {
        constructor() {
            super(...arguments);
            this.quit = () => {
                main_root_1.close(this);
            };
            this.trigger = () => {
                AudioTest.flag = !AudioTest.flag;
                if (!AudioTest.flag) {
                    sound_1.BGMusic.setMuted(true);
                } else {
                    sound_1.BGMusic.setMuted(false);
                }
                this.props.flag = AudioTest.flag;
                this.paint();
            };
            this.playBGM = index => {
                if (AudioTest.flag) {
                    if (AudioTest.bgmList[index]) {
                        sound_1.BGMusic.play(AudioTest.bgmList[index], 1, true, 5000, "linear");
                    }
                }
            };
            this.playAudio = index => {
                if (AudioTest.flag) {
                    if (AudioTest.audioList[index]) {
                        sound_2.SoundResMgr.play(AudioTest.audioList[index], 1);
                    }
                }
            };
        }
        setProps(props, oldprop) {
            super.setProps(props, oldprop);
            this.props = this.props ? this.props : {};
            this.props.bgmList = AudioTest.bgmList;
            this.props.audioList = AudioTest.audioList;
        }
        innerAudioPlay() {
            // if (innerAudio) {
            //     innerAudio.play();
            //     return;
            // }
            const resTab = new res_mgr_1.ResTab();
            const path = "app_a/music/bjm_01.mp3";
            resTab.load("RES_AUDIO_BUFFER", path, []).then(res => {
                const buffer = res.link;
                if (!innerAudio) {
                    innerAudio = new inner_audio_impl_1.PiInnerAudio(buffer);
                    innerAudio.startTime = 10;
                    innerAudio.playbackRate = 1;
                    innerAudio.loop = true;
                    innerAudio.onCanplay(() => {
                        console.log("innerAudio onCanplay");
                    });
                    innerAudio.onPlay(() => {
                        console.log("innerAudio onPlay");
                    });
                    innerAudio.onPlay(() => {
                        console.log("innerAudio onPlay1");
                    });
                    innerAudio.onPause(() => {
                        console.log("innerAudio onPause");
                    });
                    innerAudio.onStop(() => {
                        console.log("innerAudio onStop");
                    });
                    innerAudio.onEnded(() => {
                        console.log("innerAudio onEnded");
                    });
                    innerAudio.onSeeked(() => {
                        console.log("innerAudio onSeeked");
                    });
                    innerAudio.onTimeUpdate(() => {
                        console.log("innerAudio onTimeUpdate ", innerAudio.currentTime);
                    });
                }
                innerAudio.play();
            });
        }
        innerAudioPause() {
            innerAudio.pause();
        }
        innerAudioStop() {
            innerAudio.stop();
        }
        innerAudioSeek() {
            innerAudio.seek(15);
        }
        innerAudioRate(rate) {
            innerAudio.playbackRate = rate;
        }
        soundPlay(i) {
            const paths = ["app_a/music/bjm_01.mp3", 'app_a/music/level_up.mp3'];
            audio_mgr_1.Sound.play(paths[i]);
        }
        addSoundVolume() {
            soundVolume += 0.1;
            if (soundVolume > 1) {
                soundVolume = 1;
            }
            audio_mgr_1.Sound.setVolume(soundVolume);
        }
        reduceSoundVolume() {
            soundVolume -= 0.1;
            if (soundVolume < 0) {
                soundVolume = 0;
            }
            audio_mgr_1.Sound.setVolume(soundVolume);
        }
        bgmPlay(i) {
            const paths = ["app_a/music/bjm_01.mp3", 'app_a/music/fb_1.mp3', 'app_a/music/fb_3.mp3'];
            audio_mgr_1.BGM.play(paths[i]);
        }
        bgmStop() {
            audio_mgr_1.BGM.stop("quadInOut", 10000);
        }
        addBgmVolume() {
            bgmVolume += 0.1;
            if (bgmVolume > 1) {
                bgmVolume = 1;
            }
            audio_mgr_1.BGM.setVolume(bgmVolume);
        }
        reduceBgmVolume() {
            bgmVolume -= 0.1;
            if (bgmVolume < 0) {
                bgmVolume = 0;
            }
            audio_mgr_1.BGM.setVolume(bgmVolume);
        }
    }
    exports.AudioTest = AudioTest;
    AudioTest.bgmList = ["app_a/music/bjm_01.mp3", 'app_a/music/fb_1.mp3', 'app_a/music/bg_2.mp3'];
    AudioTest.audioList = ["app_a/music/bjm_01.mp3", 'app_a/music/level_up.mp3', 'app_a/music/open_box.mp3'];
    AudioTest.flag = true;
});