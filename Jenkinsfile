pipeline {
  agent { label 'UbuntuVM' }
  parameters {
    gitParameter name: 'TAG', 
                 type: 'PT_TAG',
                 defaultValue: 'master'
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
                  userRemoteConfigs: [[url: 'https://github.com/atolab/eclipse-zenoh-python.git']]
                ])
      }
    }
    stage('Release build') {
      steps {
        sh '''
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
