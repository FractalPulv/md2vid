import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api';

const useFileLoader = () => {
    const [fileFrontmatter, setFileFrontmatter] = useState([]);
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        console.log('Fetching file frontmatter...');
        invoke('get_all_files_frontmatter')
            .then((response) => {
                console.log('File frontmatter fetched:', response);
                
                // Parse the JSON response if necessary
                let parsedResponse;
                try {
                    parsedResponse = JSON.parse(response);
                } catch (e) {
                    console.error('Error parsing response:', e);
                    setLoading(false);
                    return;
                }

                console.log('Parsed response:', parsedResponse);

                setFileFrontmatter(parsedResponse);
                setLoading(false);
            })
            .catch((error) => {
                console.error('Error fetching file frontmatter:', error);
                setLoading(false);
            });
    }, []);

    return { fileFrontmatter, loading };
}

export default useFileLoader;
