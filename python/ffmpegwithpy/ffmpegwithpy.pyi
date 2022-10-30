def get_filename(filepath: str, file_format: str) -> str:
    """Get file name after processing

    Args:
        filepath (str): filepath
        file_format (str): Format you want to process

    Returns:
        str: File name after processing
    """
    ...

class FFmpeg:
    """Run FFmpeg

    This module runs FFmpeg based on the input and performs file conversion and encoding.

    Todo:
        * Cover all arguments
    """

    def __init__(self, filepath: str, file_format: str) -> None: ...
    def __new__(cls) -> None: ...
    def ffmpeg_cmd(self) -> str: ...
    def ffmpeg(self) -> None: ...
