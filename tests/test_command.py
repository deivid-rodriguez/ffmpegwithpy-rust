from ffmpegwithpy import FFmpeg, get_filename


def test_get_filename():
    after_filename = get_filename("test.mp4", "mp3")
    assert after_filename == "test.mp3"


def test_FFmpeg_ffmpeg_cmd():
    assert FFmpeg("test.mp4", "mp3").ffmpeg_cmd() == "ffmpeg test.mp4 test.mp3"
