 ($loop_counter==1)  ($emprc == 0)  ($inwork =~ /.*\.zip$/i) 
				use POSIX;
				$validsheetcount=0;
				$validpagecount=0;
				$validpackagecount=0;
				
				$validpackagecount = `cat $pre.extraccomb$post | wc -l`;
				$validsheetcount = ceil($validpackagecount/2);
				$validpagecount = $validpackagecount;
				
					# set input module to call in GMC command
					if ($gmc_input_type =~ /xml/i)
					{
					$gmc_input_module = "XMLDataInput1";
					}
					elsif ($gmc_input_type =~ /std/i)
					{
					$gmc_input_module = "QTL2";
					}
					else
					{
					die "Invalid \$gmc_input_type value: $gmc_input_type - Expected xml or std.";
					}
					
					$inbasename =~ /LGN[TP]\.FIN\.GNW0000\.(.{5})/;
					$template_Type = $1;
					print "File Type: $template_Type\n";
					
					# get chunk name from the current input file.
					if ($loop_curfile =~ /.*chnk\_(\d+)/)
					{
					$gmc_file_chunk = $1;
					}
					print "\$gmc_file_chunk: $gmc_file_chunk\n";
					print "\$loop_curfile: $loop_curfile\n";
					
				>>FILE_START
				-CONTROLFILE=/apps/dialogue/${Dialogue_Version}/key
				-MSGRESOURCE=/apps/dialogue/${Dialogue_Version}/MsgResource_en-us.dat
				-PACKAGEFILE=${usrparms}${pub_file}
				-MESSAGEFILE=$pre.exmsgs01
				-FILEMAP=DD:INPUT,$inwork
				-FILEMAP=DD:EXTRACT,$pre.extraccomb$post
				-FILEMAP=DD:OUTPDF,$pre.outpdf01
				-RUNMODE=PRODUCTION
				-TRACKIN=DISABLE
				-TRACKOUT=NONE
				-REPORT=NONE
				-MSG=1850,W
				>>FILE_END
				 ($app_process_name eq "reprint_process")  ($msite eq "lgn")  ($printer =~ /xerox|oce/i)  ($printer eq "pro")  ($msite eq "chg")  ($msite eq "dal")  ($emprc == 0)  ($msite eq "wcw")  ($msite eq "thu")  ($msite eq "hdp")  ($printer =~ /xerox/i)  ($printer eq "xerox")  ($printer eq "pro")  ($printer eq "oce")  ($printer =~ /xerox|oce/i)  ($printer eq "pro")  ($lvl eq "t")  ($household eq "y")  ($verimove eq "y")  ($pivot_track eq "y")  ($pivot_proof eq "y")  ($pivot_archive eq "y")  ($pivot_report eq "y") 
				print "
				site=$site
				psite=$psite
				csite=$csite
				msite=$msite
				asite=$asite
				location=$my_location
				location_code=$my_location_code
				printer=$my_printer
				printer_type=$my_printer_type
				doc_composer=$my_comp_type
				household=$my_household
				verimove=$my_verimove
				pivot_track=$my_pivot_track
				pivot_proof=$my_pivot_proof
				\n";
				
				use POSIX;
				# TODO: Add functionality to parse a trailer record
				$validsheetcount=0;
				$validpagecount=0;
				$validpackagecount=0;
				
				# determine input type for reconciliation type.
				open(my $IFH, "<", "$inwork") or die "Cannot open file $inwork - $!";
				if ($my_input_type =~ /pdf/i)
				{
				skip; #$validpackagecount = `cat $pre.extraccomb$post | wc -l`;
				}
				else
				{
				while (my $line = <$IFH>)
				{
				# get xml trailer values
				if ($my_input_type =~ /xml/i)
				{
				if ($line =~ /<audit>(\d+)<\/audit>/i)
				{
				$validpackagecount = $1;
				}
				}
				# get delimited trailer values
				elsif ($my_input_type =~ /pdf/i)
				{
				$validpackagecount = `cat $pre.extraccomb$post | wc -l`;
				}
				else
				{
				if ($line =~ /^TRAILER.(\d+)/i)
				{
				$validpackagecount = $1;
				}
				}
				}
				}
				close($IFH);
				# trim trailer from file
				if ($my_input_type !~ /xml/i)
				{
				`cp ${inwork} ${inwork}.ORIG`;
				`sed -i '/^TRAILER/d' ${inwork}`;
				}
				$validsheetcount = ceil($validpackagecount/2);
				$validpagecount = $validpackagecount;
				 ($doc_composer eq "gmc")  ($gmc_input_type =~ /pdf/i)  ($household eq "y")  ($verimove eq "y")  ($msort eq "y") 