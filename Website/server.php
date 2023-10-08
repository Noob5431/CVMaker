<?php
header('Access-Control-Allow-Origin: *');
header('Access-Control-Allow-Headers: *');
header('Content-Type: text/event-stream');
header('Cache-Control: no-cache');
foreach($_POST as $data => $data){
    //$data=$_POST["userMessage"];
}
    echo "data: {$data} \n\n";
    flush();
?>