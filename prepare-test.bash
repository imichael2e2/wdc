
tmpdir="./wdctmp"

# rm -rf $tmpdir

mkdir -p $tmpdir


function internel_get_chrome_mver() {
    if [[ -x /usr/bin/chromium ]]; then
	bin_path="/usr/bin/chromium"
    fi
    if [[ -x /usr/bin/chromium-browser ]]; then
	bin_path="/usr/bin/chromium-browser"
    fi
    echo $($bin_path --version)
}

function internal_get_download_url() {
    drv_name="$1"
    dl_url="xxx"
    
    if [[ $drv_name =~ "gecko" ]]; then
	case "$(uname -s)" in
	    Linux*) dl_url="https://github.com/mozilla/geckodriver/releases/download/v0.30.0/geckodriver-v0.30.0-linux64.tar.gz";;
	esac
    fi

    if [[ $drv_name =~ "chrome" ]]; then
	chrome_mver=$(internel_get_chrome_mver)
	if [[ ! -z $chrome_mver ]]
	then
	    # echo $chrome_mver
	    if [[ $chrome_mver =~ "114" ]]; then
		chrome_exactver="113.0.5672.63"
	    fi
	    if [[ $chrome_mver =~ "113" ]]; then
		chrome_exactver="114.0.5735.16"
	    fi
	    if [[ $chrome_mver =~ "112" ]]; then
		chrome_exactver="112.0.5615.49"
	    fi
	    # echo "DEBUG,$chrome_exactver" >&2
	fi
	case "$(uname -s)" in
	    Linux*) dl_url="https://chromedriver.storage.googleapis.com/$chrome_exactver/chromedriver_linux64.zip";;
	esac
    fi

    echo $dl_url
}

function prepare() {
    drv_name="$1"
    drv_archive="$2"
    drv_bin="$3"

    pkill $drv_name

    drv_url=$(internal_get_download_url $drv_name)

    # echo $drv_url
    
    if [ ! -x $drv_bin ]
    then
	echo "downloading '$drv_name'..."
	curl --location --silent $drv_url --output $drv_archive
	if [[ ! $? -eq 0 ]]; then
	    printf "download failed: %d\n" $? && exit 1
	fi

	echo "extracting '$drv_name'..."
	if [[ $drv_archive =~ ".tgz" ]]
	then
	    # echo "use tar"
	    tar --extract --overwrite --file $drv_archive -C $tmpdir
	fi
	if [[ $drv_archive =~ ".zip" ]]
	then
	    # echo "use unzip"
	    unzip -o -q $drv_archive -d $tmpdir
	fi
    fi
    if [[ ! $? -eq 0 ]]; then
	printf "extract failed: %d\n" $? && exit 1
    fi


    echo "bringing up '$drv_name'..."
    drv_args=""
    
    if [[ $drv_name =~ "gecko" ]]; then
	# drv_args="--log trace"
	drv_args="--log fatal"
    fi
    if [[ $drv_name =~ "chrome" ]]; then
	# drv_args="--log-level=ALL"
	drv_args="--log-level=SEVERE"
    fi
    # echo $drv_args   
    $drv_bin $drv_args &>"$tmpdir/$drv_name.log" &
    if [[ ! $? -eq 0 ]]; then
	printf "bringup failed: %d\n" $? && exit 1
    fi

    echo "'$drv_name' is ready!"
}


userarg=$1

function print_help_msg() {
    cat <<EOF
Usage: bash prepare-cargo-test [command]

Put WebDriver servers in ready position, before "cargo test" command.

help		Display help message
reset		Bring down all WebDriver servers and clean up temporary files
all		Prepare for all supported drivers
		(kill and restart drivers if running)
gecko		Prepare for GeckoDriver
chrome		Prepare for ChromeDriver

EOF
}

case $userarg in
    *help)
	print_help_msg
	;;
    reset)
	pkill geckodriver
	pkill chromedriver
	rm -rf "$tmpdir" # with caution
	test $? -eq 0 && echo "reset done!"
	;;
    all)
	prepare "geckodriver" "$tmpdir/geckodriver.tgz" "$tmpdir/geckodriver"
	prepare "chromedriver" "$tmpdir/chromedriver.zip" "$tmpdir/chromedriver"
	test $? -eq 0 && echo "prepare done!"
        ;;
esac
    
