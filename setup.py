from distutils.core import setup
import py2exe

setup(
	console=['display.py'],
	options = {
        'py2exe': {
            'optimize': 2,
            'packages' :  ['plotly'],
        }
    }
)