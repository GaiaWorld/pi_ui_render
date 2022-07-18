_$pi.define("app_a/audio_test/audio_test.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./audio_test.vue.tpl", "pi_sys/media/inner_audio_impl", "pi_common/ui/main_root", "pi_utils/res/sound", "pi_utils/res/res_mgr", "pi_utils/res/audio_mgr"], function (require, exports, module, direct_1, audio_test_vue_tpl_1, inner_audio_impl_1, main_root_1, sound_1, res_mgr_1, audio_mgr_1) {
    "use strict";

    exports.initMeta = void 0;
    let innerAudio;
    let soundVolume = 1;
    let bgmVolume = 1;
    class AudioTest {
        constructor() {
            this.bgmList = ["bjm_01.mp3", 'fb_1.mp3', 'bg_2.mp3'];
            this.soundList = ["collect_coin.mp3", 'level_up.mp3', 'open_box.mp3'];
            this.innerTestList = ['播放', '暂停', '停止', 'seek', '加速', '正常', '减速'];
            this.soundMgrList = ['播放音效1', '播放音效2', '音效音量+', '音效音量-', '播放背景1', '播放背景2', '播放背景3', '停止背景播放', '背景音量+', '背景音量-'];
            this.flag = true;
            this.quit = () => {
                main_root_1.close(this);
            };
            this.trigger = () => {
                this.flag = !this.flag;
                if (!this.flag) {
                    sound_1.BGMusic.setMuted(true);
                } else {
                    sound_1.BGMusic.setMuted(false);
                }
                this.flag = this.flag;
            };
            this.playBGM = index => {
                if (this.flag) {
                    if (this.bgmList[index]) {
                        sound_1.BGMusic.play('app_a/music/' + this.bgmList[index], 1, true, 5000, "linear");
                    }
                }
            };
            this.playAudio = index => {
                if (this.flag) {
                    if (this.soundList[index]) {
                        sound_1.SoundResMgr.play('app_a/music/' + this.soundList[index], 1);
                    }
                }
            };
        }
        innerTest(num) {
            switch (num) {
                case 0:
                    this.innerAudioPlay();
                    break;
                case 1:
                    this.innerAudioPause();
                    break;
                case 2:
                    this.innerAudioStop();
                    break;
                case 3:
                    this.innerAudioSeek();
                    break;
                case 4:
                    this.innerAudioRate(2);
                    break;
                case 4:
                    this.innerAudioRate(1);
                    break;
                case 4:
                    this.innerAudioRate(0.5);
                    break;
            }
        }
        innerAudioPlay() {
            const resTab = new res_mgr_1.ResTab();
            resTab.load("RES_AUDIO_BUFFER", 'app_a/music/bjm_01.mp3', []).then(res => {
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
        soundMgr(num) {
            switch (num) {
                case 0:
                    this.soundPlay(0);
                    break;
                case 1:
                    this.soundPlay(1);
                    break;
                case 2:
                    this.addSoundVolume();
                    break;
                case 3:
                    this.reduceSoundVolume();
                    break;
                case 4:
                    this.bgmPlay(0);
                    break;
                case 5:
                    this.bgmPlay(1);
                    break;
                case 6:
                    this.bgmPlay(2);
                    break;
                case 7:
                    this.bgmStop();
                    break;
                case 8:
                    this.addBgmVolume();
                    break;
                case 9:
                    this.reduceBgmVolume();
                    break;
            }
        }
        soundPlay(i) {
            audio_mgr_1.Sound.play('app_a/music/' + this.soundList[i]);
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
            audio_mgr_1.BGM.play('app_a/music/' + this.bgmList[i], "quadInOut", 10000, "quadInOut", 10000);
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
    exports.default = AudioTest;
    exports.initMeta = () => {
        let _$tpl = "app_a/audio_test/audio_test.vue.tpl.ts",
            _$cssPath = "app_a/audio_test/audio_test.vue.wcss",
            _$cssHash = 3119178135;
        AudioTest["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: audio_test_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(AudioTest, ["flag", "bgmList", "soundList", "innerTestList", "soundMgrList"]);
    direct_1.addField(AudioTest, ['quit', 'trigger', 'playBGM', 'playAudio']);
});