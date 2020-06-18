pipeline {
  agent { label 'MacMini' }
  parameters {
    gitParameter name: 'TAG', 
                 type: 'PT_TAG',
                 description: 'A Git tag (default: ${BRANCH_NAME} or ${env.BRANCH_NAME})',
                 defaultValue: '${BRANCH_NAME}'
  }

  stages {
    stage('Checkout Git TAG') {
      steps {
        cleanWs()
        checkout([$class: 'GitSCM',
                  branches: [[name: "${params.TAG}"]],
                  doGenerateSubmoduleConfigurations: false,
                  extensions: [[$class: 'SubmoduleOption',
                    depth: 1,
                    disableSubmodules: false,
                    parentCredentials: false,
                    recursiveSubmodules: true,
                    reference: '',
                    shallow: true,
                    trackingSubmodules: true]],
                  gitTool: 'Default',
                  submoduleCfg: [],
                  userRemoteConfigs: [[url: 'https://github.com/eclipse-zenoh/zenoh-python.git']]
                ])
      }
    }
    stage('Release build') {
      steps {
        sh '''
          . ~/.zshrc
          export PLAT_NAME=macosx-10.9-x86_64
          for PYTHON_ENV in zenoh-cp35 zenoh-cp36 zenoh-cp37 zenoh-cp38; do
            conda activate ${PYTHON_ENV}
            make
          done

          make all-cross
        '''
      }
    }
    stage('Deploy to download.eclipse.org') {
      steps {
        sshagent ( ['projects-storage.eclipse.org-bot-ssh']) {
          sh '''
          ssh genie.zenoh@projects-storage.eclipse.org mkdir -p /home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${TAG}
          scp dist/*.whl  genie.zenoh@projects-storage.eclipse.org:/home/data/httpd/download.eclipse.org/zenoh/zenoh-python/${TAG}/
          '''
        }
      }
    }
    stage('Deploy on pypi.org') {
      steps {
        sh '''
          . ~/.zshrc
          python3 -m twine upload --repository eclipse-zenoh dist/*.whl
        '''
      }
    }
  }

  post {
    success {
        archiveArtifacts artifacts: 'dist/*.whl', fingerprint: true
    }
  }
}
