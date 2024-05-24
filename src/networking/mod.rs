pub mod fetch {
    pub async fn get_html_page() -> serde_json::Value {
        let html_page = "<!DOCTYPE html PUBLIC '-//W3C//DTD XHTML 1.0 Transitional//EN' 'http://www.w3.org/TR/xhtml1/DTD/xhtml1-transitional.dtd'>
            <html xmlns='http://www.w3.org/1999/xhtml'>
            <head>
            <meta http-equiv='Content-Type' content='text/html; charset=utf-8' />
            <title>ZOMBO</title>
            <link href='zombo.css' rel='stylesheet' type='text/css' />
            <link href='https://maxcdn.bootstrapcdn.com/font-awesome/4.2.0/css/font-awesome.min.css' rel='stylesheet'>
            <!--responsive stuff added here-->
            <meta name='viewport' content='width=device-width,initial-scale=1,maximum-scale=1,user-scalable=no'>
            <meta http-equiv='X-UA-Compatible' content='IE=edge,chrome=1'>
            <meta name='HandheldFriendly' content='true'>

            </head>
            <body>
            <div align='center'>
              <p><br />
              </p>
              <p><img src='images/zombocom.png' width='1199' height='217' alt='Zombocom' longdesc='http://zombo.com' /></p>
            </div>
            <div align='center'>
              <!--z stuff here -->
               <div class='animate-flicker'>
              <p><img src='images/pngwheel.png' class='rotate thefade'  /></p>
            </div>
            </div>
            <!--Z stuff here -->
            <audio loop src='zombo_words.mp3' type='audio/mpeg'></audio>
            <button id='button'>
              <i class='fa fa-volume-up'></i>
            <script>
            const button = document.querySelector('#button');
            const icon = document.querySelector('#button > i');
            const audio = document.querySelector('audio');

            button.addEventListener('click', () => {
              if (audio.paused) {
                audio.volume = 0.2;
                audio.play();
                icon.classList.remove('fa-volume-up');
                icon.classList.add('fa-volume-off');

              } else {
                audio.pause();
                icon.classList.remove('fa-volume-off');
                icon.classList.add('fa-volume-up');
              }
              button.classList.add('fade');
            });
            </script>

            </body>
            </html>
            ";

        return serde_json::json!({
            "status": 200,
            "text": html_page
        });
    }
}
